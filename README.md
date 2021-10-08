# flyte project

it's a lib written in pure rust that helps you manage files and directories across multiple filesystems.

it's still work in progress, expect bugs and breaking changes.

## available filesystems
- `local` 
- `s3` (depends on: `rusoto_s3`)

## usage
you can either use filesystems directly or use the `FilesystemChain` to perform same operation across multiple filesystems.

## license
see [LICENSE.md](LICENSE.md)
