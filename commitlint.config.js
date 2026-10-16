module.exports = {
  // The built-in ignores (merge commits, reverts, fixup!/squash!, bare
  // versions) let a malformed header through, and some match body lines too.
  // main squash-merges from the PR title and body, so none of them apply.
  defaultIgnores: false,
  extends: ["@commitlint/config-conventional"],
  rules: {
    "body-max-line-length": [0, "always", Infinity],
    "footer-max-line-length": [0, "always", Infinity],
    "body-leading-blank": [0, "always"],
  },
};
