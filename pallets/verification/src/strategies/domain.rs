// This file is part of Anagolay Network.

// Copyright (C) 2019-2022 Anagolay Network.

use crate::types::*;
use codec::Encode;
use core::{convert::TryInto, marker::PhantomData};
use frame_support::sp_std::{str, vec::Vec};
use sp_runtime::offchain::{http, Duration};

/// Structure representing the verification strategy for a domain using DNS TXT record.
///
/// # Type Arguments
/// T: the frame system configuration used as associated type for the implemented trait
/// [`VerificationStrategy`]
#[derive(Clone)]
pub struct DnsVerificationStrategy<T: crate::Config> {
  _marker: PhantomData<T>,
}

/// Internal implementation of the strategy
impl<T: crate::Config> DnsVerificationStrategy<T> {
  /// Produce the verification key for the context
  ///
  /// # Arguments
  /// * context - The source [`VerificationContext`]
  ///
  /// # Return
  /// The verification key Bytes
  fn produce_key(&self, holder: &T::AccountId, context: &VerificationContext) -> Result<Bytes, crate::Error<T>> {
    match context {
      VerificationContext::UrlForDomain(_, domain) => {
        let mut identifier = Vec::new();
        identifier.append(&mut domain.clone().into_inner());
        identifier.append(&mut holder.encode());
        let mut cid = T::VerificationKeyGenerator::generate(holder, context, identifier)?;
        let mut key = "anagolay-domain-verification=".as_bytes().to_vec();
        key.append(&mut cid);
        key
          .try_into()
          .map_err(|_| crate::Error::<T>::VerificationKeyGenerationError)
      }
      VerificationContext::UrlForDomainWithSubdomain(_, domain, subdomain) => {
        let mut identifier = Vec::new();
        identifier.append(&mut domain.clone().into_inner());
        identifier.append(&mut subdomain.clone().into_inner());
        identifier.append(&mut holder.encode());
        let mut cid = T::VerificationKeyGenerator::generate(holder, context, identifier)?;
        let mut key = "anagolay-domain-verification.".as_bytes().to_vec();
        key.append(&mut subdomain.clone().into_inner());
        key.append(&mut "=".as_bytes().to_vec());
        key.append(&mut cid);
        key
          .try_into()
          .map_err(|_| crate::Error::<T>::VerificationKeyGenerationError)
      }
      _ => Err(crate::Error::<T>::VerificationKeyGenerationError),
    }
  }

  /// Parse the DNS over HTTP response to find the given key in the TXT records. The expected format
  /// of the json response is illustrated here:
  /// https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-json/
  ///
  /// # Arguments
  /// * body - The answer to a TXT request
  /// * key - The verification key from the request
  ///
  /// # Return
  /// Some [`VerificationStatus`] if the verification was successful (`Success` or `Failed`), None
  /// if the parsing failed
  fn verify_doh_response(body: &str, key: &Bytes) -> Option<VerificationStatus> {
    use lite_json::json::JsonValue;
    let key = key.clone().into_inner();
    let val = lite_json::parse_json(body);
    match val.ok()? {
      JsonValue::Object(obj) => {
        let (_, v) = obj.into_iter().find(|(k, _)| k.iter().copied().eq("Answer".chars()))?;
        match v {
          JsonValue::Array(obj) => obj.iter().find_map(|v| match v {
            JsonValue::Object(obj) => match obj.iter().find(|(k, _)| k.iter().copied().eq("data".chars())) {
              Some((_, JsonValue::String(record))) => {
                // Verify string equality, deal with json value potentially wrapped in quotes
                let matching = key.iter().zip(record.iter()).filter(|&(a, b)| *a == *b as u8).count();
                let matching_wrapped = key
                  .iter()
                  .zip(record[1..record.len() - 1].iter())
                  .filter(|&(a, b)| *a == *b as u8)
                  .count();
                let verified = matching == key.len() && matching == record.len() ||
                  matching_wrapped == key.len() && matching_wrapped == record.len() - 2;
                if verified {
                  Some(VerificationStatus::Success)
                } else {
                  let mut error_msg = "Unexpected key is found: '".as_bytes().to_vec();
                  error_msg.append(&mut record.iter().map(|b| *b as u8).collect());
                  error_msg.append(&mut "'".as_bytes().to_vec());
                  Some(VerificationStatus::Failure(error_msg.into()))
                }
              }
              _ => None,
            },
            _ => None,
          }),
          _ => None,
        }
      }
      _ => None,
    }
  }
}

impl<T: crate::Config> Default for DnsVerificationStrategy<T> {
  fn default() -> Self {
    DnsVerificationStrategy::<T> {
      _marker: PhantomData::<T>::default(),
    }
  }
}

/// Implementation of the [`VerificationStrategy`] trait for [`DnsVerificationStrategy`]
impl<T: crate::Config> VerificationStrategy for DnsVerificationStrategy<T> {
  type Config = T;
  type VerificationError = http::Error;

  /// Creates a new [`VerificationRequest`]
  ///
  /// # Arguments
  /// - holder: The verification holder
  /// - context: The [`VerificationContext`] to check
  /// - action: The [`VerificationAction`] the end user has chosen to perform
  ///
  /// # Return
  /// A [`VerificationRequest`] from the given context and action
  fn new_request(
    &self,
    holder: <Self::Config as frame_system::Config>::AccountId,
    context: VerificationContext,
    action: VerificationAction,
  ) -> Result<VerificationRequest<<Self::Config as frame_system::Config>::AccountId>, crate::Error<T>> {
    let key = self.produce_key(&holder, &context)?;
    Ok(
      VerificationRequest::<<Self::Config as frame_system::Config>::AccountId> {
        context,
        action,
        holder,
        status: VerificationStatus::Waiting,
        key,
        id: None,
      },
    )
  }

  /// Defines whether a [`VerificationContext`] is supported or not
  ///
  /// # Arguments
  /// - context: The [`VerificationContext`] to check
  /// - action: The [`VerificationAction`] the end user has chosen to perform
  ///
  /// # Return
  /// True if the context is supported by this strategy, false otherwise
  fn supports(&self, context: &VerificationContext, action: &VerificationAction) -> bool {
    match context {
      VerificationContext::UrlForDomain(_, _) | VerificationContext::UrlForDomainWithSubdomain(_, _, _) => match action
      {
        VerificationAction::DnsTxtRecord => true,
      },
      _ => false,
    }
  }

  /// Performs an HTTP call to check the required criterion to pass the verification
  ///
  /// # Arguments
  /// - request: The `VerificationRequest` to verify
  ///
  /// # Return
  /// A `VerificationStatus` resulting from the verification
  fn verify(
    &self,
    request: &VerificationRequest<<Self::Config as frame_system::Config>::AccountId>,
  ) -> Result<VerificationStatus, Self::VerificationError> {
    // Perform a DNS over HTTPS resolution to retrieve the TXT records of the domain
    let get = match &request.context {
      VerificationContext::UrlForDomain(_, domain) | VerificationContext::UrlForDomainWithSubdomain(_, domain, _) => {
        let mut url = "https://cloudflare-dns.com/dns-query?name=".as_bytes().to_vec();
        url.append(&mut domain.clone().into_inner());
        url.append(&mut "&type=txt".as_bytes().to_vec());
        url
      }
      _ => "".into(),
    };
    let get = http::Request::get(str::from_utf8(&get).unwrap_or_default()).add_header("accept", "application/dns-json");

    let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
    let pending = get.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
    let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;

    // Next we want to fully read the response body as a str
    let body = response.body().collect::<Vec<u8>>();
    let body = str::from_utf8(&body).map_err(|_| http::Error::Unknown)?;

    // Parse the DoH response to find the exact same key in a TXT record
    match Self::verify_doh_response(body, &request.key) {
      Some(verified) => Ok(verified),
      None => Err(http::Error::Unknown),
    }
  }
}
