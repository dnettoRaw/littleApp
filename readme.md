# little App

Esse sera o primeiro app feito en rust e typescript, o objetivo e um app que possa gerir uma base de dados em sqlite, e que possa se sincronizar com um servidor.

# quer testar? vem eu te mostro por onde comeca

## baixando a ultima release

### [github releases](https://github.com/dnettoRaw/littleApp/releases)

## compilando o projeto voce mesmi

    git clone https://github.com/dnettoRaw/littleApp.git

> instala as dependencias

    npm i

> copia o .env.example para .env e coloca as variaveis de ambiente 

> para a versao dev 

    npm t:dev

> para a versao executavel

    npm t:build
 
[npm](https://www.npmjs.com/), [node](https://nodejs.org/en/) e [rust](https://www.rust-lang.org/pt-BR/) sao necessarios para rodar o projeto

# todo
- [x] npm commands in readme 
- [ ] interface user 
- [ ] backend functions
- [ ] api functions
- [ ] testes unitarios

## Commands
| npm              |                    Description                    |
|------------------|---------------------------------------------------|
|`npm start`       |  start your app in dev
|`npm run dev`     |  start dev with hot refresh
|`npm run build`   |  build your project, for stactic render, this command auto update patch verion in packege
|`npm run preview` |  start `vite preview` for server your pre-build app (only host compiled)      
|`npm run t`       |  to run tauri commands from main folder
|`npm run t:dev`   |  shortcut for tauri dev
|`npm run t:build` |  shortcut for tauri build

## bug

if your want to use tauri app in dev and you get a blank screen please change `"devPath": "http://127.0.0.1:3000"` to `"devPath": "http://localhost:3000"` but you need to change that again every time wen you want to make a new build. ([bug found here](https://github.com/tauri-apps/tauri/issues/1140#issuecomment-848874865))

## little tips

to optmize you binary you can try to change `opt-level` value to `z` or `s` in `/src-tauri/cargo.toml` this can reduce the final size of you binary 

## Some links

[official tauri discord](https://discord.gg/tauri)

[tauri studio](https://tauri.studio)

[tauri github](https://github.com/tauri-apps)

[tauri rust doc](https://docs.rs/tauri/1.0.0-rc.3/tauri/index.html)
