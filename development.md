# For Developers among us

## commits

The gitlab job [`test`](./.devops/ci/gitlab/jobs/test.yml) is configured to run with the [`.rules-run-if-source-code-is-changed`](./.devops/ci/gitlab/utils/rules.yml#20) rule which triggers it in one of two cases:

1. if the commit message starts with `[sc build]`
2. any changes in rust files are made in the `node`, `pallets`, `runtime` directories

## sccache

when you first time create the workspace you will need to do this:

```bash
# to get the envs
direnv allow

# stop the server which started without the env variables and it's using the local cache
sccache --stop-server

# to start the server and verify that it is using the aws as a storge
sccache --start-server; sccache --show-stats
```

## gitlab and jobs

### Extending the jobs

Links https://docs.gitlab.com/ee/ci/yaml/yaml_optimization.html#use-extends-to-reuse-configuration-sections, https://docs.gitlab.com/ee/ci/yaml/#extends , https://docs.gitlab.com/ee/ci/yaml/yaml_optimization.html#use-extends-and-include-together
With extend pay attention on the rules of merging.

```
SCCACHE_START_SERVER=1 SCCACHE_NO_DAEMON=1 RUST_LOG=sccache=trace SCCACHE_LOG=debug sccache

sccache --stop-server && SCCACHE_START_SERVER=1 SCCACHE_NO_DAEMON=1 RUST_LOG=sccache=trace SCCACHE_LOG=debug sccache
```
