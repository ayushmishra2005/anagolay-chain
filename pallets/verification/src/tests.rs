// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

//! Tests for the module.

use super::{
  mock::{Call, *},
  *,
};
use crate::{
  consts::MaxVerificationRequestsPerContextGet,
  types::{offchain::*, *},
};
use codec::Decode;
use core::convert::TryInto;
use frame_support::{traits::ReservableCurrency, *};
use sp_core::{
  offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
  sr25519, Pair,
};
use sp_runtime::testing::TestXt;

type Extrinsic = TestXt<Call, ()>;

fn mock_account(ss58: &str) -> sr25519::Public {
  let (pair, _) = sr25519::Pair::from_string_with_seed(ss58, None).unwrap();
  pair.public()
}

fn mock_request<T>(
  holder: T::AccountId,
  context: VerificationContext,
  action: VerificationAction,
) -> VerificationRequest<T::AccountId>
where
  T: crate::Config,
{
  VerificationRequest::<T::AccountId> {
    context,
    action,
    holder,
    status: VerificationStatus::Pending,
    key: "anagolay-domain-verification=test".into(),
    id: None,
  }
}

#[test]
fn request_verification_error_on_context_submitted_twice() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;
    let request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());

    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder, context.clone(), request);

    let res = VerificationTest::request_verification(origin, context, action);
    assert_noop!(res, Error::<Test>::VerificationAlreadyIssued);
  });
}

#[test]
fn request_verification_allow_resubmit_failed_request() {
  let holder = mock_account("//Alice");
  new_test_ext(vec![(holder, 100)]).execute_with(|| {
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;
    let mut request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());
    request.status = VerificationStatus::Failure("an error description".into());

    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(
      holder.clone(),
      context.clone(),
      request.clone(),
    );

    let res = VerificationTest::request_verification(origin.clone(), context.clone(), action.clone());
    assert_ok!(res);
    assert_eq!(
      AccountIdsByVerificationContext::<Test>::get(context.clone()),
      vec![holder.clone()]
    );

    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder.clone(), context.clone(), request);

    let res = VerificationTest::request_verification(origin, context.clone(), action.clone());
    assert_ok!(res);
    assert_eq!(
      AccountIdsByVerificationContext::<Test>::get(context.clone()),
      vec![holder.clone()]
    );
  });
}

#[test]
fn request_verification_error_on_cannot_reserve_registration_fee() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;

    let res = VerificationTest::request_verification(origin, context, action);
    assert_noop!(res, Error::<Test>::CannotReserveRegistrationFee);
  });
}

#[test]
fn request_verification_domain_verification_requested() {
  let holder = mock_account("//Alice");
  new_test_ext(vec![(holder, 100)]).execute_with(|| {
    // To emit events, we need to be past block 0
    System::set_block_number(1);

    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;

    let res = VerificationTest::request_verification(origin, context.clone(), action);
    assert_ok!(res);

    let event_record: frame_system::EventRecord<_, _> = System::events().pop().unwrap();
    let generic_event: crate::mock::Event = event_record.event;
    let pallet_event: crate::Event<Test> = generic_event.try_into().unwrap();

    let (holder, request) = match pallet_event {
      crate::Event::VerificationRequested(holder, request) => (holder, request),
      _ => panic!("unexpected error"),
    };
    let stored_request = VerificationRequestByAccountIdAndVerificationContext::<Test>::get(holder, context).unwrap();

    assert_eq!(
      stored_request, request,
      "The request in the storage must match the one dispatched by the Event::VerificationRequested"
    );
    assert_eq!(
      "anagolay-domain-verification=d4",
      std::str::from_utf8(&request.key.into_inner()).unwrap(),
      "The computed key is incorrect"
    );
    assert_eq!(
      VerificationStatus::Waiting,
      request.status,
      "The computed status is incorrect"
    );
  });
}

#[test]
fn request_verification_subdomain_verification_requested() {
  let holder = mock_account("//Alice");
  new_test_ext(vec![(holder, 100)]).execute_with(|| {
    // To emit events, we need to be past block 0
    System::set_block_number(1);

    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomainWithSubdomain(
      "https://sub.anagolay.network".into(),
      "anagolay.network".into(),
      "sub".into(),
    );
    let action = VerificationAction::DnsTxtRecord;

    let res = VerificationTest::request_verification(origin, context.clone(), action);
    assert_ok!(res);

    let event_record: frame_system::EventRecord<_, _> = System::events().pop().unwrap();
    let generic_event: crate::mock::Event = event_record.event;
    let pallet_event: crate::Event<Test> = generic_event.try_into().unwrap();

    let (holder, request) = match pallet_event {
      crate::Event::VerificationRequested(holder, request) => (holder, request),
      _ => panic!("unexpected error"),
    };
    let stored_request = VerificationRequestByAccountIdAndVerificationContext::<Test>::get(holder, context).unwrap();

    assert_eq!(
      stored_request, request,
      "The request in the storage must match the one dispatched by the Event::VerificationRequested"
    );
    assert_eq!(
      "anagolay-domain-verification.sub=8a",
      std::str::from_utf8(&request.key.into_inner()).unwrap(),
      "The computed key is incorrect"
    );
    assert_eq!(
      VerificationStatus::Waiting,
      request.status,
      "The computed status is incorrect"
    );
  });
}

#[test]
fn perform_verification_error_no_such_verification_request() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;
    let request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());

    let res = VerificationTest::perform_verification(origin, request);
    assert_noop!(res, Error::<Test>::NoSuchVerificationRequest);
  });
}

#[test]
fn perform_verification_error_invalid_verification_status() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;
    let mut request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());
    request.status = VerificationStatus::Failure("an error description".into());
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder, context.clone(), request.clone());

    let res = VerificationTest::perform_verification(origin, request);
    assert_noop!(res, Error::<Test>::InvalidVerificationStatus);
  });
}

#[test]
fn perform_verification_domain_verification_from_non_holder() {
  let holder = mock_account("//Alice");
  let verifier = mock_account("//Bob");
  new_test_ext(vec![(holder, 100)]).execute_with(|| {
    // To emit events, we need to be past block 0
    System::set_block_number(1);

    let origin = mock::Origin::signed(verifier);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;

    let mut request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder, context.clone(), request.clone());

    request.id = Some("not used".into());

    let res = VerificationTest::perform_verification(origin, request);
    assert_ok!(res);

    let event_record: frame_system::EventRecord<_, _> = System::events().pop().unwrap();
    let generic_event: crate::mock::Event = event_record.event;
    let pallet_event: crate::Event<Test> = generic_event.try_into().unwrap();

    let (verifier, request) = match pallet_event {
      crate::Event::VerificationRequested(verifier, request) => (verifier, request),
      _ => panic!("unexpected error"),
    };
    let stored_request = VerificationRequestByAccountIdAndVerificationContext::<Test>::get(holder, context).unwrap();

    assert_eq!(
      stored_request, request,
      "The request in the storage must match the one dispatched by the Event::VerificationRequested"
    );
    assert_ne!(verifier, holder, "The holder must not be the same as the verifier");
    assert_eq!(Some("not used".into()), request.id, "The id was not correctly updated");
    assert_eq!(
      VerificationStatus::Pending,
      request.status,
      "The computed status is incorrect"
    );
  });
}

#[test]
fn perform_verification_domain_offchain_process() {
  let mut t = new_test_ext(Vec::new());
  let (pool, pool_state) = testing::TestTransactionPoolExt::new();
  t.register_extension(TransactionPoolExt::new(pool));
  let (offchain, oc_state) = testing::TestOffchainExt::new();
  t.register_extension(OffchainWorkerExt::new(offchain));

  {
    let mut oc_state = oc_state.write();
    oc_state.expect_request(testing::PendingRequest {
      method: "GET".into(),
      uri: "https://cloudflare-dns.com/dns-query?name=anagolay.network&type=txt".into(),
      headers: vec![("accept".to_string(), "application/dns-json".to_string())],
      response: Some(
        br#"{
        "Status": 0,
        "TC": false,
        "RD": true,
        "RA": true,
        "AD": true,
        "CD": false,
        "Question": [
          {
            "name": "anagolay.network.",
            "type": 16
          }
        ],
        "Answer": [
          {
            "name": "anagolay.network.",
            "type": 16,
            "TTL": 1726,
            "data": "anagolay-domain-verification=test"
          }
        ]
      }
      "#
        .to_vec(),
      ),
      sent: true,
      ..Default::default()
    });
  }

  let holder = mock_account("//Alice");
  t.execute_with(|| {
    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;

    let mut request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());
    request.status = VerificationStatus::Pending;

    let data = VerificationIndexingData {
      verifier: holder.clone(),
      request: request.clone(),
    };

    // when
    let res = VerificationTest::process_pending_verification(data.clone());
    assert_ok!(res);
    let mut successful_verification = data.clone();
    successful_verification.request.status = VerificationStatus::Success;

    // then
    let tx = pool_state.write().transactions.pop().unwrap();
    assert!(pool_state.read().transactions.is_empty());
    let tx = Extrinsic::decode(&mut &*tx).unwrap();
    assert_eq!(tx.signature, None);
    assert_eq!(
      tx.call,
      Call::VerificationTest(crate::Call::submit_verification_status {
        verification_data: successful_verification
      })
    );
  })
}

#[test]
fn perform_submit_verification_status_failure_from_non_holder() {
  let holder = mock_account("//Alice");
  let verifier = mock_account("//Bob");
  new_test_ext(vec![(holder, 100), (verifier, 10)]).execute_with(|| {
    // To emit events, we need to be past block 0
    System::set_block_number(1);

    <Test as Config>::Currency::reserve(&holder, Test::REGISTRATION_FEE).unwrap();
    assert_eq!(Balances::reserved_balance(&holder), Test::REGISTRATION_FEE);
    assert_eq!(Balances::free_balance(&holder), 100 - Test::REGISTRATION_FEE);
    assert_eq!(Balances::reserved_balance(&verifier), 0);
    assert_eq!(Balances::free_balance(&verifier), 10);

    let origin = frame_system::RawOrigin::None.into();

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;

    let request: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder.clone(), context.clone(), action.clone());
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder, context.clone(), request.clone());

    let data = VerificationIndexingData {
      verifier: verifier.clone(),
      request: VerificationRequest {
        status: VerificationStatus::Failure("an error description".into()),
        ..request.clone()
      },
    };

    let res = VerificationTest::submit_verification_status(origin, data);
    assert_ok!(res);

    let event_record: frame_system::EventRecord<_, _> = System::events().pop().unwrap();
    let generic_event: crate::mock::Event = event_record.event;
    let pallet_event: crate::Event<Test> = generic_event.try_into().unwrap();

    let (verifier, holder, request) = match pallet_event {
      crate::Event::VerificationFailed(verifier, holder, request, _error) => (verifier, holder, request),
      _ => panic!("unexpected error"),
    };
    let stored_request = VerificationRequestByAccountIdAndVerificationContext::<Test>::get(holder, context).unwrap();

    assert_eq!(
      stored_request, request,
      "The request in the storage must match the one dispatched by the Event::VerificationRequested"
    );
    assert_eq!(
      VerificationStatus::Failure("an error description".into()),
      request.status,
      "The computed status is incorrect"
    );
    assert_eq!(Balances::reserved_balance(&holder), 0);
    assert_eq!(Balances::free_balance(&holder), 100 - Test::REGISTRATION_FEE);
    assert_eq!(Balances::reserved_balance(&verifier), 0);
    assert_eq!(Balances::free_balance(&verifier), 10 + Test::REGISTRATION_FEE);
  });
}

#[test]
fn rpc_get_request_pagination() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder1 = mock_account("//Alice");
    let holder2 = mock_account("//Bob");

    let context1 = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let context2 = VerificationContext::UrlForDomain("https://kelp.digital".into(), "kelp.digital".into());
    let action = VerificationAction::DnsTxtRecord;

    let mut request1: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder1.clone(), context1.clone(), action.clone());
    request1.status = VerificationStatus::Failure("an error description".into());
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder1, context1.clone(), request1.clone());

    let mut request2: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder1.clone(), context2.clone(), action.clone());
    request2.status = VerificationStatus::Success;
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder1, context2.clone(), request2.clone());

    let mut request3: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder2.clone(), context1.clone(), action.clone());
    request3.status = VerificationStatus::Failure("an error description".into());
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder2, context1.clone(), request3.clone());

    let mut request4: VerificationRequest<<Test as frame_system::Config>::AccountId> =
      mock_request::<Test>(holder2.clone(), context2.clone(), action.clone());
    request4.status = VerificationStatus::Failure("an error description".into());
    VerificationRequestByAccountIdAndVerificationContext::<Test>::insert(holder2, context2.clone(), request4.clone());

    let holders: BoundedVec<<Test as frame_system::Config>::AccountId, MaxVerificationRequestsPerContextGet<Test>> =
      vec![holder1.clone(), holder2.clone()].try_into().unwrap();
    AccountIdsByVerificationContext::<Test>::insert(context1.clone(), holders.clone());
    AccountIdsByVerificationContext::<Test>::insert(context2.clone(), holders);

    let res = VerificationTest::get_requests(vec![], None, None, 0, 10);
    assert_eq!(res.len(), 4);
    let res = VerificationTest::get_requests(vec![context1.clone()], None, None, 0, 10);
    assert_eq!(res.len(), 2);
    let res = VerificationTest::get_requests(vec![context1.clone(), context2.clone()], None, None, 0, 10);
    assert_eq!(res.len(), 4);
    let res = VerificationTest::get_requests(vec![], Some(VerificationStatus::Success), None, 0, 10);
    assert_eq!(res.len(), 1);
    let res = VerificationTest::get_requests(
      vec![],
      Some(VerificationStatus::Failure("anything".into())),
      None,
      0,
      10,
    );
    assert_eq!(res.len(), 3);
    let res = VerificationTest::get_requests(
      vec![],
      Some(VerificationStatus::Failure("anything".into())),
      Some(holder1),
      0,
      10,
    );
    assert_eq!(res.len(), 1);
    let res = VerificationTest::get_requests(vec![], None, Some(holder2), 1, 10);
    assert_eq!(res.len(), 1);
  });
}
