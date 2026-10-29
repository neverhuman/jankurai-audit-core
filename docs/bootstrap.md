# Cold-host bootstrap for locked Git sources

`Cargo.lock` pins every jankurai component to an exact commit
(`git+https://github.com/neverhuman/<repo>.git?rev=<sha>`), and the workspace
`[patch]` entries name the same repositories through the `www.github.com`
alias. A host with an empty Cargo git cache therefore fetches from both
`github.com` and `www.github.com`.

## Known state

The GitHub publishing mirror is not a dependable source. When last probed,
anonymous `git ls-remote` of both `https://github.com/neverhuman/<repo>.git`
and `https://www.github.com/neverhuman/<repo>.git` returned HTTP 403 with
`Repository '<account>/<repo>' is disabled`. Re-enabling that account is an
owner-only action on GitHub; nothing in this repository can do it, and this
document does not claim the mirror is healthy. Run `scripts/git-source-route.sh
probe` to re-check.

## Trusted route

Fetch from the authoritative forge instead, without changing any pin:

```sh
export JANKURAI_GIT_MIRROR_BASE=https://forge.example/git/org   # your forge, https, no trailing slash
scripts/git-source-route.sh run -- cargo metadata --locked --format-version 1
scripts/git-source-route.sh run -- bash scripts/ci-local.sh required
```

The script exports `GIT_CONFIG_COUNT/KEY/VALUE` `url.<base>/.insteadOf` rewrites
for the owners found in `Cargo.lock`, for both host spellings, in the child
process only. It writes no global or repository git config, does not edit
`Cargo.toml` or `Cargo.lock`, and Cargo still checks that the fetched commit is
the locked one, so revs, tags and checksums are unchanged. Forge credentials, if
the forge needs any, come from your normal credential helper and are never
stored by the script. If Cargo's own git client is not used, set
`CARGO_NET_GIT_FETCH_WITH_CLI=true` so the rewrite applies.

`scripts/git-source-route.sh probe` reports, per locked source, whether it is
reachable anonymously (no prompt, no credential helper); with the mirror base
set it also probes the trusted route. Anonymous results show public
accessibility; a forge that needs a login will report unreachable there, and
that is separate from authenticated access, which the probe does not test.

`scripts/git-source-route-test.sh` (part of the required lane) proves offline
that the rewrite covers every locked source and leaves the lock file untouched.
