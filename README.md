# Solana Monopoly Game
![Static Badge](https://img.shields.io/badge/anchor-0.29-blue)
![Static Badge](https://img.shields.io/badge/solana-1.17-green)
![Static Badge](https://img.shields.io/badge/dioxus-1.17-orange)
![Static Badge](https://img.shields.io/badge/tailwindcss-3.4-blue)
![Static Badge](https://img.shields.io/badge/wallet-phantom-violet)

- Both frontend and program are written in Rust, the frontend uses Dioxus, and the program uses Anchor.
- Use Tailwind CSS for responsive layout, supporting both PC and mobile.
- Use Phantom, the most used Solana wallet.

<img src="images/phone1.jpg" width="15%"><img src="images/phone3.jpg" width="15%">
<img src="images/phone4.jpg" width="15%"><img src="images/phone5.jpg" width="15%">
<img src="images/phone6.jpg" width="15%"><img src="images/phone7.jpg" width="15%">
<img src="images/pc1.png" width="90%">

**Program Address（devnet）:** ```GMDedNzaiCffFmBNVBDUzd6Ub6XLQ4xhoWfswBRmYqbG```  
**Play Online:** 
- **4everland:** https://solana-monopoly-game-frontend-deploy-1j2d63jo-ame1ma.4everland.app/
- **Github:** https://ame1ma.github.io/solana-monopoly-game-frontend-deploy/

### Play
- According to https://en.wikipedia.org/wiki/Monopoly_(game) , simplified  
- Requires browser installation [Phantom Wallet Extension](https://phantom.app/)  
- If the RPC address is not specified, the devnet is used by default，https://api.devnet.solana.com  
### Deploy
1. Install Anchor：https://www.anchor-lang.com/docs/installation  
2. Install Dioxus：https://dioxuslabs.com/learn/0.5/getting_started  
3. Install Tailwind Css：https://tailwindcss.com/docs/installation  
4. Modify the provider.wallet in `Anchor.toml`.  
5. Solana program use `anchor build` build, use `anchor deploy` deploy.
6. Frontend needs enter the frontend folder，use `dx build --release` build.

