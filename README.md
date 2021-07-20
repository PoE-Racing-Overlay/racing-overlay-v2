# PoE Racing Overlay

## Project setup

Follow the steps at https://tauri.studio/en/docs/getting-started/intro to get your environment set-up

Then look into [Tauri Usage](https://tauri.studio/en/docs/usage/intro)

If you are slightly insane (like me!) you can also use the docker image `pxslip/tauri-vue` to get a tauri+vue dev environment easily. You do need to setup an X11 server on the host machine to display the output! On windows check out [This Guide](https://dev.to/darksmile92/run-gui-app-in-linux-docker-container-on-windows-host-4kde). [Setup](https://medium.com/@mreichelt/how-to-show-x11-windows-within-docker-on-mac-50759f4b65cb) is similar on MacOS and even easier on [Linux](https://l10nn.medium.com/running-x11-applications-with-docker-75133178d090)

### Compiles and hot-reloads for development

```
yarn tauri:serve
npm run tauri:serve
```

### Compiles and minifies for production

```
yarn tauri:build
npm run tauri:build
```

### Lints and fixes files

```
yarn lint
```

### Customize configuration

#### Vue Configuration

See [Configuration Reference](https://cli.vuejs.org/config/).

#### Tauri Configuration

See [Configuration Reference](https://tauri.studio/en/docs/api/config)
