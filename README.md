# findfriends

rust 写的第一个项目，根据用户star的仓库查看相近的用户

## usage

```bash
cargo run  -- --username yourbutterfly --token <github_token>

# ...
# 
# fly51fly: 222
# xrkk: 235
# tututu-patch: 238
# ufwt: 241
# agnosticlines: 251
# 5l1v3r1: 256
# famasoon: 258
# D4rkD0g: 267
# lifa123: 291
# YourButterfly: 505

```

注意， 跳过了40k star以上的项目，忽略了相同star少于10的用户
## todo

 - [ ] 代理
 - [ ] 多线程
 - [ ] 重试
 - [ ] ...
  
