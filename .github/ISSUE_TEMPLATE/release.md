---
name: Release issue template
about: Tracking issue for new releases
title: Selendra Release checklist
---
# Release Checklist

This is the release checklist for Selendra new version. **All** following
checks should be completed before publishing a new release of the
Selendra runtime or client.

### Runtime Releases

These checks should be performed on the codebase prior to forking to a release-candidate branch.

- [ ] Verify [`spec_version`](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#spec-version) has been incremented since the last release for any native runtimes from any existing use on public (non-private) networks. If the runtime was published (release or         pre-release), either the `spec_version` or `impl` must be bumped.

- [ ] Verify previously [completed migrations](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#old-migrations-removed) are removed for any public (non-private/test) networks.

- [ ] Verify pallet and [extrinsic ordering](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#extrinsic-ordering) has stayed the same. Bump `transaction_version` if not.

- [ ] Verify new extrinsics have been correctly whitelisted/blacklisted for
    [proxy filters](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#proxy-filtering).

- [ ] Verify [benchmarks](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#benchmarks) have been updated for any modified
    runtime logic.

The following checks can be performed after we have forked off to the release-
candidate branch or started an additional release candidate branch (rc-2, rc-3, etc)

- [ ] Verify [new migrations](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#new-migrations) successfully, and the
    runtime state is correctly updated for any public (non-private/test) networks.
- [ ] Check with the Signer's team to make sure metadata update QR are lined up
- [ ] Push runtime upgrade to Selendra testnet and verify network stability.

### All Releases

- [ ] Check that the new client versions have [run on the network](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#burn-in)
    without issue for 12+ hours on >75% of our validator nodes.
- [ ] Check that a draft release has been created at
    https://github.com/selendra/selendra/releases with relevant [release
    notes](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#release-notes)
- [ ] Check that [build artifacts](https://github.com/selendra/selendra/blob/master/docs/release-checklist.md#build-artifacts) have been added to the draft-release
- [ ] Check that all items listed in the [milestone](https://github.com/selendra/selendra/milestones) are included in the release.


# Branching Strategy
Inspired by `Brave Branch Strategy` 
```
prod => releases to production
master => releases to staging
dev => releases to development
```
1. production releases should only be made after have been able to test exactly what we're going to release on stage. So these should always be a PR from `master` to `prod` that's basically `"make production === stage"`. These are the only PRs that should go to `prod`.

2. therefore a merge to `master` should only happen when we think the feature is ready to release.

3. when starting a piece of work, create a branch off `master` and keep adding commits there until it's ready to release.

4. to test the code in a real environment, either: 
- a. merge that branch to dev - but don't delete the feature branch. Repeatedly merge the feature that feature branch to dev as work progresses. Merges to dev do not require PRs. OR 
- b. manually initiate the `"Deploy to Development"` github action selecting that branch - this will deploy just those changes to development.

5. In-development QA of this feature should happen on the development environment.

6. when it's good to go merge the feature branch to `master` - with a PR and security review if required. 

7. Do not merge until all reviews are completed.
then, after checking on the staging environment (including QA regression testing if needed) PR a production release as per step 1.

8. now and again we will reset dev to match `master` just to keep the history tidy.

