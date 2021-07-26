# Workers KV – Desktop GUI

> A cross-platform Desktop application for exploring Workers KV Namespace data

Built with:

* [Svelte](https://svelte.dev) – UI framework
* [Svelte Kit](https://kit.svelte.dev) – UI build system
* [Tauri](https://tauri.studio) – Desktop Application framework
* [Redis](https://redis.io/) – The backing database / cache

## Install

> **Note:** Pre-built applications are available on the [Releases](https://github.com/cloudflare/workerskv.gui/releases) page!

You must have [Redis installed locally](https://redis.io/download) to use `localhost` servers.

***Local Development***

Please follow the [Tauri Getting Started Guide](https://tauri.studio/en/docs/getting-started/intro#steps) to setup your system with the Rust toolchain.

Once complete, this application is built like a typical Node.js application – however, [`pnpm`](https://pnpm.io/) is used for package management.

```sh
$ pnpm install
$ pnpm run build
```

> **Note:** You may use `yarn` or `npm`, but only a `pnpm` lockfile is included.

## Screenshots

***Setup or Manage Connections***

<img
  width="480"
  src="shots/connect.png"
  alt="setup connection details"
/>

***Viewing Namespace Keys***

<img
  width="480"
  src="shots/viewer.png"
  alt="view key properties and value"
/>
