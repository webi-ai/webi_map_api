# webi_map_api
api for webI that uses sudograph


*optimized*



to run use 

```bash
npm run dfx-deploy-local for local replica deployment

or

npm run dfx-deploy-ic for IC deployment
```



# faq

this error:
```
Error: The post-build step failed for canister 'rrkah-fqaaa-aaaaa-aaaaq-cai' with an embedded error: No such file or directory (os error 2)
```
need to install wasm ic optimizer in project directory

```bash
cargo install ic-cdk-optimizer --root target
```

then run optimize script
```
npm run dfx-optimize-graphql
```
