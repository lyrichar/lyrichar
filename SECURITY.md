# Security Policy

## Reporting

Thank you for taking the time to responsibly disclose any problems you find.

**Do not file public issues as they are open for everyone to see!**

All security vulnerabilities in `lyrichar` should be reported by email
to [security@lyrichar.app][Security Email].
Your report will be acknowledged within 24 hours, and you will receive a more
detailed response within 48 hours indicating the next steps in handling your report.

You can encrypt your report using our public key:
[`7B0748A06BAFD6924F3DDAFC8B7D9874F16320BB`][Security Key].
This key is also available on [MIT's Key Server][MIT Key Server]
and [reproduced below](#security-key).

After the initial reply to your report, the core team will try to keep you
informed of the progress being made towards a fix and official announcement.
These updates will be sent at least every five days. In reality, this is
more likely to be every 24-48 hours.

## Disclosure Policy

`lyrichar` has a 5-step disclosure process:

1. The security report is received and is assigned a primary handler.
   This person will coordinate the fix and release process.

2. The problem is confirmed and a list of all affected versions is determined.

3. Code is audited to find any potential similar problems.

4. Fixes are prepared for all releases which are still under maintenance.
   These fixes are not committed to the public repository but rather
   held locally pending the announcement.

5. On the embargo date, the changes are pushed to the public repository
   and new builds are deployed.

This process can take some time, especially when coordination is required
with maintainers of other projects. Every effort will be made to handle
the issue in as timely a manner as possible, however it is important that
we follow the release process above to ensure that the disclosure is handled
in a consistent manner.

## Security Key

```text
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEaqrWPBYJKwYBBAHaRw8BAQdAjVGnL2dJauLf4RKtBXXfKv587+C/m4LofTVL
ovYFOKC0K0x5cmljaGFyIChzZWN1cml0eSkgPHNlY3VyaXR5QGx5cmljaGFyLmFw
cD6IkAQTFgoAOBYhBHsHSKBrr9aSTz3a/It9mHTxYyC7BQJqqtY8AhsDBQsJCAcC
BhUKCQgLAgQWAgMBAh4BAheAAAoJEIt9mHTxYyC7bP4A/Aluw49q7V6prO+iGaCM
4qW9YzlCYQKqu7F8FxaKAKukAQCULYO4U6QjVbPoDDQ2bpc1e8DdLkpbfDN29xTp
ZYZNCLg4BGqq1jwSCisGAQQBl1UBBQEBB0AVcCPvP8HzPPBPDFGXCndrOJBTYsq2
cPAiGrs2BWreFQMBCAeIeAQYFgoAIBYhBHsHSKBrr9aSTz3a/It9mHTxYyC7BQJq
qtY8AhsMAAoJEIt9mHTxYyC7X2wA+QEp8RRPr3zwjHT9V0LEBlGn5NsQM5+imwcN
EGqCNowqAQDJGhDCNoWi6nTlx+aU2Ia2Vpo6Lj2P9UShqMRB6V4NDQ==
=IC31
-----END PGP PUBLIC KEY BLOCK-----
```

## Attribution

This Security Policy is adapted from [Rust's Security Policy][Rust Security Policy].

[Security Email]: mailto:security@lyrichar.app
[Security Key]: https://lyrichar.app/keys/security
[MIT Key Server]: https://pgp.mit.edu/pks/lookup?op=index&search=0x7B0748A06BAFD6924F3DDAFC8B7D9874F16320BB
[Rust Security Policy]: https://rust-lang.org/policies/security
