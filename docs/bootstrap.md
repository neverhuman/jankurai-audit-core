# Cold-host bootstrap for locked Git sources

`Cargo.lock` pins every jankurai component to an exact commit
(`git+https://github.com/neverhuman/<repo>.git?rev=<sha>`), and the workspace
`[patch]` entries name the same repositories through the `www.github.com`
alias. A host with an empty Cargo git cache therefore fetches from both
`github.com` and `www.github.com`.

## The GitHub alias is not a dependable source

The GitHub publishing mirror has been observed failing for some of the pinned
repositories: an anonymous fetch of
`https://www.github.com/neverhuman/jankurai-tools-kernel.git` returned HTTP 403
with `Repository '<account>/<repo>' is disabled`, which fails `cargo fetch`
outright. Re-enabling that account is an owner-only action on GitHub; nothing
in this repository can do it, and nothing here claims the mirror is healthy.

Its state also changes without notice — on a later probe some of the same
repositories answered anonymously while others did not — so do not read a past
result as current. Always re-probe rather than assume:

```sh
scripts/git-source-route.sh probe
```

`probe` reports every locked source twice and never conflates the two:
`anonymous` fetches with prompts and credential helpers disabled, so it
measures what the public can actually get; `authenticated` fetches the same
repository on the trusted mirror using the host's normal credential helpers.
A mirror that needs a login is expected to be unreachable anonymously, and that
says nothing about the public mirror's health.

## Trusted route

Fetch from the authoritative forge instead, without changing any pin:

```sh
export JANKURAI_GIT_MIRROR_BASE=https://forge.example/git/org   # your forge, https, no trailing slash
scripts/git-source-route.sh run -- cargo fetch --locked
scripts/git-source-route.sh run -- bash scripts/ci-local.sh required
```

The base names the directory that holds the repositories; the GitHub owner is
dropped, so `<base>/<repo>.git` must exist on the forge. Ask the forge
administrator for the base rather than guessing it — it is a host specific and
is deliberately not committed here.

The script exports `GIT_CONFIG_COUNT/KEY/VALUE` `url.<base>/.insteadOf` rewrites
for the owners found in `Cargo.lock`, for both host spellings, in the child
process only. It writes no global or repository git config, does not edit
`Cargo.toml` or `Cargo.lock`, and Cargo still checks that the fetched commit is
the locked one, so revs, tags and checksums are unchanged. Forge credentials, if
the forge needs any, come from your normal credential helper and are never
stored by the script. Set `CARGO_NET_GIT_FETCH_WITH_CLI=true` so Cargo fetches
through the git CLI, which is what reads the rewrite.

## Proving a cold host really bootstraps

```sh
JANKURAI_GIT_MIRROR_BASE=https://forge.example/git/org scripts/git-source-route.sh verify
```

`verify` fetches into a freshly created, empty `CARGO_HOME` (under `.agent/`,
never `/tmp`, removed afterwards; override with `JANKURAI_COLD_CACHE_DIR`), then
checks that **every** locked 40-hex commit id is present in that cold cache and
that `Cargo.lock` is byte-identical afterwards. It needs the network and a forge
login, so it is not part of the required lane; run it on a host when you want
proof rather than a claim. A base that does not serve the pins makes it report
`MISSING <rev>` and exit non-zero.

`scripts/git-source-route-test.sh` (part of the required lane) is the offline
half: it proves the rewrite and `mirror-url` cover every locked source on both
host spellings, that an unset or non-https base is refused, that no git config
is written, and that the lock file is untouched.
