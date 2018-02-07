/*
 * coin.rs
 *
 * Copyright 2018 Standard Mining
 *
 * Available to be used and modified under the terms of the MIT License.
 *
 * THE SOFTWARE IS PROVsymbolED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 * WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Represents the various cryptocurrencies supported by this crate.

use self::Coin::*;

/// The actual enum that represents a cryptocurrency.
/// This enum is not intended to be matched exhaustively,
/// as new coins may be added in the future.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Coin {
    /// Bitcoin, symbol "BTC", URL: https://bitcoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "BTC"))]
    Bitcoin,

    /// Testnet (all coins), symbol "TEST"
    #[cfg_attr(feature = "serde", serde(rename = "TEST"))]
    Testnet,

    /// Litecoin, symbol "LTC", URL: https://litecoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "LTC"))]
    Litecoin,

    /// Dogecoin, symbol "DOGE", URL: https://github.com/dogecoin/dogecoin
    #[cfg_attr(feature = "serde", serde(rename = "DOGE"))]
    Dogecoin,

    /// Reddcoin, symbol "RDD"
    #[cfg_attr(feature = "serde", serde(rename = "RDD"))]
    Reddcoin,

    /// Dash, symbol "DSH", URL: https://github.com/dashpay/dash
    #[cfg_attr(feature = "serde", serde(rename = "DSH"))]
    Dash,

    /// Peercoin, symbol "PPC", URL: https://peercoin.net/
    #[cfg_attr(feature = "serde", serde(rename = "PPC"))]
    Peercoin,

    /// Namecoin, symbol "NMC", URL: http://namecoin.info/
    #[cfg_attr(feature = "serde", serde(rename = "NMC"))]
    Namecoin,

    /// Feathercoin, symbol "FTC", URL: https://www.feathercoin.com/
    #[cfg_attr(feature = "serde", serde(rename = "FTC"))]
    Feathercoin,

    /// Counterparty, symbol "XCP", URL: http://counterparty.io/
    #[cfg_attr(feature = "serde", serde(rename = "XCP"))]
    Counterparty,

    /// Blackcoin, symbol "BLK", URL: http://blackcoin.co/
    #[cfg_attr(feature = "serde", serde(rename = "BLK"))]
    Blackcoin,

    /// NuShares, symbol "NSR", URL: https://nubits.com/nushares/introduction
    #[cfg_attr(feature = "serde", serde(rename = "NSR"))]
    NuShares,

    /// NuBits, symbol "NBT"
    #[cfg_attr(feature = "serde", serde(rename = "NBT"))]
    NuBits,

    /// Mazacoin, symbol "MZC"
    #[cfg_attr(feature = "serde", serde(rename = "MZC"))]
    Mazacoin,

    /// Viacoin, symbol "VIA"
    #[cfg_attr(feature = "serde", serde(rename = "VIA"))]
    Viacoin,

    /// ClearingHouse, symbol "XCH"
    #[cfg_attr(feature = "serde", serde(rename = "XCH"))]
    ClearingHouse,

    /// Rubycoin, symbol "RBY"
    #[cfg_attr(feature = "serde", serde(rename = "RBY"))]
    Rubycoin,

    /// Groestlcoin, symbol "GRS"
    #[cfg_attr(feature = "serde", serde(rename = "GRS"))]
    Groestlcoin,

    /// Digitalcoin, symbol "DGC"
    #[cfg_attr(feature = "serde", serde(rename = "DGC"))]
    Digitalcoin,

    /// Cannacoin, symbol "CCN"
    #[cfg_attr(feature = "serde", serde(rename = "CCN"))]
    Cannacoin,

    /// DigiByte, symbol "DGB"
    #[cfg_attr(feature = "serde", serde(rename = "DGB"))]
    DigiByte,

    /// Monacoin, symbol "MONA"
    #[cfg_attr(feature = "serde", serde(rename = "MONA"))]
    Monacoin,

    /// Clams, symbol "CLAM"
    #[cfg_attr(feature = "serde", serde(rename = "CLAM"))]
    Clams,

    /// Primecoin, symbol "XPM"
    #[cfg_attr(feature = "serde", serde(rename = "XPM"))]
    Primecoin,

    /// Neoscoin, symbol "NEOS"
    #[cfg_attr(feature = "serde", serde(rename = "NEOS"))]
    Neoscoin,

    /// Jumbucks, symbol "JBS"
    #[cfg_attr(feature = "serde", serde(rename = "JBS"))]
    Jumbucks,

    /// ziftrCOIN, symbol "ZRC"
    #[cfg_attr(feature = "serde", serde(rename = "ZRC"))]
    ZiftCoin,

    /// Vertcoin, symbol "VTC"
    #[cfg_attr(feature = "serde", serde(rename = "VTC"))]
    Vertcoin,

    /// NXT, symbol "NXT"
    #[cfg_attr(feature = "serde", serde(rename = "NXT"))]
    NXT,

    /// Burst, symbol "BURST"
    #[cfg_attr(feature = "serde", serde(rename = "BURST"))]
    Burst,

    /// MonetaryUnit, symbol "MUE"
    #[cfg_attr(feature = "serde", serde(rename = "MUE"))]
    MonetaryUnit,

    /// Zoom, symbol "ZOOM"
    #[cfg_attr(feature = "serde", serde(rename = "ZOOM"))]
    Zoom,

    /// Vpncoin, symbol "VPN"
    #[cfg_attr(feature = "serde", serde(rename = "VPN"))]
    Vpncoin,

    /// Canada eCoin, symbol "CDN", URL: https://github.com/Canada-eCoin/
    #[cfg_attr(feature = "serde", serde(rename = "CDN"))]
    CanadaeCoin,

    /// ShadowCash, symbol "SDC"
    #[cfg_attr(feature = "serde", serde(rename = "SDC"))]
    ShadowCash,

    /// ParkByte, symbol "PKB", URL: https://github.com/parkbyte/
    #[cfg_attr(feature = "serde", serde(rename = "PKB"))]
    ParkByte,

    /// Pandacoin, symbol "PND"
    #[cfg_attr(feature = "serde", serde(rename = "PND"))]
    Pandacoin,

    /// StartCOIN, symbol "START"
    #[cfg_attr(feature = "serde", serde(rename = "START"))]
    StartCoin,

    /// MOIN, symbol "MOIN", URL: https://discovermoin.com
    #[cfg_attr(feature = "serde", serde(rename = "MOIN"))]
    MOIN,

    /// Expanse, symbol "EXP", URL: http://www.expanse.tech/
    #[cfg_attr(feature = "serde", serde(rename = "EXP"))]
    Expanse,

    /// Decred, symbol "DCR", URL: https://decred.org/
    #[cfg_attr(feature = "serde", serde(rename = "DCR"))]
    Decred,

    /// NEM, symbol "XEM", URL: https://github.com/NemProject
    #[cfg_attr(feature = "serde", serde(rename = "XEM"))]
    NEM,

    /// Particl, symbol "PART", URL: https://particl.io/
    #[cfg_attr(feature = "serde", serde(rename = "PART"))]
    Particl,

    /// Argentum, symbol "ARG", URL: http://www.argentum.io
    #[cfg_attr(feature = "serde", serde(rename = "ARG"))]
    Argentum,

    /// Shreeji, symbol "SHR", URL: https://github.com/SMJBIT/SHREEJI
    #[cfg_attr(feature = "serde", serde(rename = "SHR"))]
    Shreeji,

    /// Global Currency Reserve (GCRcoin), symbol "GCR"
    #[cfg_attr(feature = "serde", serde(rename = "GCR"))]
    GcrCoin,

    /// Novacoin, symbol "NVC", URL: https://github.com/novacoin-project/novacoin
    #[cfg_attr(feature = "serde", serde(rename = "NVC"))]
    Novacoin,

    /// Asiacoin, symbol "AC", URL: https://github.com/AsiaCoin/AsiaCoinFix
    #[cfg_attr(feature = "serde", serde(rename = "AC"))]
    Asiacoin,

    /// Bitcoindark, symbol "BTCD", URL: https://github.com/jl777/btcd
    #[cfg_attr(feature = "serde", serde(rename = "BTCD"))]
    Bitcoindark,

    /// Dopecoin, symbol "DOPE", URL: https://github.com/dopecoin-dev/DopeCoinV3
    #[cfg_attr(feature = "serde", serde(rename = "DOPE"))]
    Dopecoin,

    /// Templecoin, symbol "TPC", URL: https://github.com/9cat/templecoin
    #[cfg_attr(feature = "serde", serde(rename = "TPC"))]
    Templecoin,

    /// AIB, symbol "AIB", URL: https://github.com/iobond/aib
    #[cfg_attr(feature = "serde", serde(rename = "AIB"))]
    AIB,

    /// EDRCoin, symbol "EDRC", URL: https://github.com/EDRCoin/EDRcoin-src
    #[cfg_attr(feature = "serde", serde(rename = "EDRC"))]
    EDRCoin,

    /// Syscoin, symbol "SYS", URL: https://github.com/syscoin/syscoin2
    #[cfg_attr(feature = "serde", serde(rename = "SYS"))]
    Syscoin,

    /// Solarcoin, symbol "SLR", URL: https://github.com/onsightit/solarcoin
    #[cfg_attr(feature = "serde", serde(rename = "SLR"))]
    Solarcoin,

    /// Smileycoin, symbol "SMLY", URL: https://github.com/tutor-web/smileyCoin
    #[cfg_attr(feature = "serde", serde(rename = "SMLY"))]
    Smileycoin,

    /// Ethereum, symbol "ETH", URL: https://ethereum.org/ether
    #[cfg_attr(feature = "serde", serde(rename = "ETH"))]
    Ethereum,

    /// Ethereum Classic, symbol "ETC", URL: https://ethereumclassic.github.io
    #[cfg_attr(feature = "serde", serde(rename = "ETC"))]
    EthereumClassic,

    /// Pesobit, symbol "PSB", URL: https://github.com/pesobitph/pesobit-source
    #[cfg_attr(feature = "serde", serde(rename = "PSB"))]
    Pesobit,

    /// Landcoin, symbol "LDCN", URL: http://landcoin.co/
    #[cfg_attr(feature = "serde", serde(rename = "LDCN"))]
    Landcoin,

    /// Bitcoinplus, symbol "XBC", URL: https://bitcoinplus.org
    #[cfg_attr(feature = "serde", serde(rename = "XBC"))]
    Bitcoinplus,

    /// Internet of People, symbol "IOP", URL: http://www.fermat.org
    #[cfg_attr(feature = "serde", serde(rename = "IOP"))]
    InternetofPeople,

    /// Nexus, symbol "NXS", URL: http://www.nexusearth.com/
    #[cfg_attr(feature = "serde", serde(rename = "NXS"))]
    Nexus,

    /// InsaneCoin, symbol "INSN", URL: http://insanecoin.com
    #[cfg_attr(feature = "serde", serde(rename = "INSN"))]
    InsaneCoin,

    /// OkCash, symbol "OK", URL: https://github.com/okcashpro/
    #[cfg_attr(feature = "serde", serde(rename = "OK"))]
    OkCash,

    /// BritCoin, symbol "BRIT", URL: https://britcoin.com
    #[cfg_attr(feature = "serde", serde(rename = "BRIT"))]
    BritCoin,

    /// Compcoin, symbol "CMP", URL: https://compcoin.com
    #[cfg_attr(feature = "serde", serde(rename = "CMP"))]
    Compcoin,

    /// Crown, symbol "CRW", URL: http://crown.tech/
    #[cfg_attr(feature = "serde", serde(rename = "CRW"))]
    Crown,

    /// BelaCoin, symbol "BELA", URL: http://belacoin.org
    #[cfg_attr(feature = "serde", serde(rename = "BELA"))]
    BelaCoin,

    /// Virtual Cash, symbol "VASH", URL: http://www.bitnet.cc/
    #[cfg_attr(feature = "serde", serde(rename = "VASH"))]
    VirtualCash,

    /// FujiCoin, symbol "FJC", URL: http://www.fujicoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "FJC"))]
    FujiCoin,

    /// MIX, symbol "MIX", URL: https://www.mix-blockchain.org/
    #[cfg_attr(feature = "serde", serde(rename = "MIX"))]
    MIX,

    /// Verge, symbol "XVG", URL: https://github.com/vergecurrency/verge/
    #[cfg_attr(feature = "serde", serde(rename = "XVG"))]
    Verge,

    /// Electronic Gulden, symbol "EFL", URL: https://egulden.org/
    #[cfg_attr(feature = "serde", serde(rename = "EFL"))]
    ElectronicGulden,

    /// ClubCoin, symbol "CLUB", URL: https://clubcoin.co/
    #[cfg_attr(feature = "serde", serde(rename = "CLUB"))]
    ClubCoin,

    /// RichCoin, symbol "RICHX", URL: https://richcoin.us/
    #[cfg_attr(feature = "serde", serde(rename = "RICHX"))]
    RichCoin,

    /// Potcoin, symbol "POT", URL: http://potcoin.com/
    #[cfg_attr(feature = "serde", serde(rename = "POT"))]
    Potcoin,

    /// Quarkcoin, symbol "QRK"
    #[cfg_attr(feature = "serde", serde(rename = "QRK"))]
    Quarkcoin,

    /// Terracoin, symbol "TRC"
    #[cfg_attr(feature = "serde", serde(rename = "TRC"))]
    Terracoin,

    /// Gridcoin, symbol "GRC"
    #[cfg_attr(feature = "serde", serde(rename = "GRC"))]
    Gridcoin,

    /// Auroracoin, symbol "AUR", URL: http://auroracoin.is/
    #[cfg_attr(feature = "serde", serde(rename = "AUR"))]
    Auroracoin,

    /// IXCoin, symbol "IXC"
    #[cfg_attr(feature = "serde", serde(rename = "IXC"))]
    IXCoin,

    /// Gulden, symbol "NLG", URL: https://Gulden.com/
    #[cfg_attr(feature = "serde", serde(rename = "NLG"))]
    Gulden,

    /// BitBean, symbol "BITB", URL: http://bitbean.org/
    #[cfg_attr(feature = "serde", serde(rename = "BITB"))]
    BitBean,

    /// Bata, symbol "BTA", URL: http://bata.io/
    #[cfg_attr(feature = "serde", serde(rename = "BTA"))]
    Bata,

    /// Myriadcoin, symbol "XMY", URL: http://myriadcoin.org
    #[cfg_attr(feature = "serde", serde(rename = "XMY"))]
    Myriadcoin,

    /// BitSend, symbol "BSD", URL: http://bitsend.info
    #[cfg_attr(feature = "serde", serde(rename = "BSD"))]
    BitSend,

    /// Unobtanium, symbol "UNO", URL: http://http://unobtanium.uno/
    #[cfg_attr(feature = "serde", serde(rename = "UNO"))]
    Unobtanium,

    /// MasterTrader, symbol "MTR", URL: https://github.com/CrypticApplications/MTR-Update/
    #[cfg_attr(feature = "serde", serde(rename = "MTR"))]
    MasterTrader,

    /// GoldBlocks, symbol "GB", URL: https://github.com/goldblockscoin/goldblocks
    #[cfg_attr(feature = "serde", serde(rename = "GB"))]
    GoldBlocks,

    /// Saham, symbol "SHM", URL: https://github.com/SahamDev/SahamDev
    #[cfg_attr(feature = "serde", serde(rename = "SHM"))]
    Saham,

    /// Chronos, symbol "CRX", URL: https://github.com/chronoscoin/Chronoscoin
    #[cfg_attr(feature = "serde", serde(rename = "CRX"))]
    Chronos,

    /// Ubiquoin, symbol "BIQ", URL: https://github.com/ubiquoin/ubiq
    #[cfg_attr(feature = "serde", serde(rename = "BIQ"))]
    Ubiquoin,

    /// Evotion, symbol "EVO", URL: https://github.com/evoshiun/Evotion
    #[cfg_attr(feature = "serde", serde(rename = "EVO"))]
    Evotion,

    /// SaveTheOcean, symbol "STO", URL: https://github.com/SaveTheOceanMovement/SaveTheOceanCoin
    #[cfg_attr(feature = "serde", serde(rename = "STO"))]
    SaveTheOcean,

    /// BigUp, symbol "BIGUP", URL: https://github.com/BigUps/
    #[cfg_attr(feature = "serde", serde(rename = "BIGUP"))]
    BigUp,

    /// GameCredits, symbol "GMC", URL: https://github.com/gamecredits-project
    #[cfg_attr(feature = "serde", serde(rename = "GMC"))]
    GameCredits,

    /// Dollarcoins, symbol "DLC", URL: https://github.com/dollarcoins/source
    #[cfg_attr(feature = "serde", serde(rename = "DLC"))]
    Dollarcoins,

    /// Zayedcoin, symbol "ZYD", URL: https://github.com/ZayedCoin/Zayedcoin
    #[cfg_attr(feature = "serde", serde(rename = "ZYD"))]
    Zayedcoin,

    /// Dubaicoin, symbol "DBIC", URL: https://github.com/DubaiCoinDev/DubaiCoin
    #[cfg_attr(feature = "serde", serde(rename = "DBIC"))]
    Dubaicoin,

    /// Stratis, symbol "STRAT", URL: http://www.stratisplatform.com
    #[cfg_attr(feature = "serde", serde(rename = "STRAT"))]
    Stratis,

    /// Shilling, symbol "SH", URL: https://github.com/yavwa/Shilling
    #[cfg_attr(feature = "serde", serde(rename = "SH"))]
    Shilling,

    /// MarsCoin, symbol "MARS", URL: http://www.marscoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "MARS"))]
    MarsCoin,

    /// Ubiq, symbol "UBQ", URL: https://github.com/Ubiq
    #[cfg_attr(feature = "serde", serde(rename = "UBQ"))]
    Ubiq,

    /// Pesetacoin, symbol "PTC", URL: http://pesetacoin.info/
    #[cfg_attr(feature = "serde", serde(rename = "PTC"))]
    Pesetacoin,

    /// Neurocoin, symbol "NRC", URL: https://neurocoin.org
    #[cfg_attr(feature = "serde", serde(rename = "NRC"))]
    Neurocoin,

    /// ARK, symbol "ARK", URL: https://ark.io
    #[cfg_attr(feature = "serde", serde(rename = "ARK"))]
    ARK,

    /// UltimateSecureCashMain, symbol "USC", URL: http://ultimatesecurecash.info
    #[cfg_attr(feature = "serde", serde(rename = "USC"))]
    UltimateSecureCashMain,

    /// Hempcoin, symbol "HMP", URL: http://hempcoin.org
    #[cfg_attr(feature = "serde", serde(rename = "HMP"))]
    Hempcoin,

    /// Linx, symbol "LINX", URL: https://mylinx.io
    #[cfg_attr(feature = "serde", serde(rename = "LINX"))]
    Linx,

    /// Ecoin, symbol "ECN", URL: https://www.ecoinsource.com
    #[cfg_attr(feature = "serde", serde(rename = "ECN"))]
    Ecoin,

    /// Denarius, symbol "DNR", URL: https://denarius.io
    #[cfg_attr(feature = "serde", serde(rename = "DNR"))]
    Denarius,

    /// Pinkcoin, symbol "PINK", URL: http://getstarted.with.pink
    #[cfg_attr(feature = "serde", serde(rename = "PINK"))]
    Pinkcoin,

    /// PiggyCoin, symbol "PIGGY", URL: https://www.piggy-coin.com/
    #[cfg_attr(feature = "serde", serde(rename = "PIGGY"))]
    PiggyCoin,

    /// Pivx, symbol "PIVX", URL: https://github.com/PIVX-Project/PIVX
    #[cfg_attr(feature = "serde", serde(rename = "PIVX"))]
    Pivx,

    /// Flashcoin, symbol "FLASH", URL: https://flashcoin.io
    #[cfg_attr(feature = "serde", serde(rename = "FLASH"))]
    Flashcoin,

    /// Zencash, symbol "ZEN", URL: https://zensystem.io
    #[cfg_attr(feature = "serde", serde(rename = "ZEN"))]
    Zencash,

    /// Putincoin, symbol "PUT", URL: https://putincoin.info
    #[cfg_attr(feature = "serde", serde(rename = "PUT"))]
    Putincoin,

    /// BitZeny, symbol "ZNY", URL: http://bitzeny.org/
    #[cfg_attr(feature = "serde", serde(rename = "ZNY"))]
    BitZeny,

    /// Unify, symbol "UNIFY", URL: http://unifycryptocurrency.com
    #[cfg_attr(feature = "serde", serde(rename = "UNIFY"))]
    Unify,

    /// StealthCoin, symbol "XST", URL: http://www.stealthcoin.com
    #[cfg_attr(feature = "serde", serde(rename = "XST"))]
    StealthCoin,

    /// Breakout Coin, symbol "BRK", URL: http://www.breakoutcoin.com
    #[cfg_attr(feature = "serde", serde(rename = "BRK"))]
    BreakoutCoin,

    /// Vcash, symbol "VC", URL: https://vcash.info
    #[cfg_attr(feature = "serde", serde(rename = "VC"))]
    Vcash,

    /// Monero, symbol "XMR", URL: https://getmonero.org/
    #[cfg_attr(feature = "serde", serde(rename = "XMR"))]
    Monero,

    /// Voxels, symbol "VOX", URL: https://www.voxelus.com
    #[cfg_attr(feature = "serde", serde(rename = "VOX"))]
    Voxels,

    /// NavCoin, symbol "NAV", URL: https://github.com/navcoindev/navcoin2
    #[cfg_attr(feature = "serde", serde(rename = "NAV"))]
    NavCoin,

    /// Factom Factoids, symbol "FCT", URL: https://github.com/FactomProject/FactomDocs/blob/master/wallet_info/wallet_test_vectors.md
    #[cfg_attr(feature = "serde", serde(rename = "FCT"))]
    FactomFactoids,

    /// Factom Entry Credits, symbol "EC", URL: https://github.com/FactomProject
    #[cfg_attr(feature = "serde", serde(rename = "EC"))]
    FactomEntryCredits,

    /// Zcash, symbol "ZEC", URL: https://z.cash
    #[cfg_attr(feature = "serde", serde(rename = "ZEC"))]
    Zcash,

    /// Lisk, symbol "LSK", URL: https://lisk.io/
    #[cfg_attr(feature = "serde", serde(rename = "LSK"))]
    Lisk,

    /// Steem, symbol "STEEM", URL: http://steem.io
    #[cfg_attr(feature = "serde", serde(rename = "STEEM"))]
    Steem,

    /// ZCoin, symbol "XZC", URL: https://zcoin.io
    #[cfg_attr(feature = "serde", serde(rename = "XZC"))]
    ZCoin,

    /// Rootstock, symbol "RSK", URL: http://www.rsk.co/
    #[cfg_attr(feature = "serde", serde(rename = "RSK"))]
    Rootstock,

    /// RealPointCoin, symbol "RPT", URL: https://github.com/MaxSmile/RealPointCoinQt
    #[cfg_attr(feature = "serde", serde(rename = "RPT"))]
    RealPointCoin,

    /// LBRY Credits, symbol "LBC", URL: https://lbry.io/
    #[cfg_attr(feature = "serde", serde(rename = "LBC"))]
    LBRYCredits,

    /// Komodo, symbol "KMD", URL: https://komodoplatform.com/
    #[cfg_attr(feature = "serde", serde(rename = "KMD"))]
    Komodo,

    /// bisq Token, symbol "BSQ", URL: http://bisq.io/
    #[cfg_attr(feature = "serde", serde(rename = "BSQ"))]
    BisqToken,

    /// Riecoin, symbol "RIC", URL: https://github.com/riecoin/riecoin
    #[cfg_attr(feature = "serde", serde(rename = "RIC"))]
    Riecoin,

    /// Ripple, symbol "XRP", URL: https://ripple.com
    #[cfg_attr(feature = "serde", serde(rename = "XRP"))]
    Ripple,

    /// Bitcoin Cash, symbol "BCH", URL: https://www.bitcoincash.org
    #[cfg_attr(feature = "serde", serde(rename = "BCH"))]
    BitcoinCash,

    /// Neblio, symbol "NEBL", URL: https://nebl.io
    #[cfg_attr(feature = "serde", serde(rename = "NEBL"))]
    Neblio,

    /// ZClassic, symbol "ZCL", URL: http://zclassic.org/
    #[cfg_attr(feature = "serde", serde(rename = "ZCL"))]
    ZClassic,

    /// Stellar Lumens, symbol "XLM", URL: https://www.stellar.org/
    #[cfg_attr(feature = "serde", serde(rename = "XLM"))]
    StellarLumens,

    /// WhaleCoin, symbol "WHL", URL: https://whalecoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "WHL"))]
    WhaleCoin,

    /// EuropeCoin, symbol "ERC", URL: https://www.europecoin.eu.org/
    #[cfg_attr(feature = "serde", serde(rename = "ERC"))]
    EuropeCoin,

    /// Diamond, symbol "DMD", URL: http://bit.diamonds
    #[cfg_attr(feature = "serde", serde(rename = "DMD"))]
    Diamond,

    /// Bytom, symbol "BTM", URL: https://bytom.io
    #[cfg_attr(feature = "serde", serde(rename = "BTM"))]
    Bytom,

    /// Biocoin, symbol "BIO", URL: https://biocoin.bio
    #[cfg_attr(feature = "serde", serde(rename = "BIO"))]
    Biocoin,

    /// Whitecoin, symbol "XWC", URL: https://www.whitecoin.info
    #[cfg_attr(feature = "serde", serde(rename = "XWC"))]
    Whitecoin,

    /// Bitcoin Gold, symbol "BTG", URL: http://www.btcgpu.org
    #[cfg_attr(feature = "serde", serde(rename = "BTG"))]
    BitcoinGold,

    /// SuperSkynet, symbol "SSN", URL: http://wwww.superskynet.org/
    #[cfg_attr(feature = "serde", serde(rename = "SSN"))]
    SuperSkynet,

    /// TOACoin, symbol "TOA", URL: http://www.toacoin.com
    #[cfg_attr(feature = "serde", serde(rename = "TOA"))]
    TOACoin,

    /// Bitcore, symbol "BTX", URL: https://bitcore.cc
    #[cfg_attr(feature = "serde", serde(rename = "BTX"))]
    Bitcore,

    /// Adcoin, symbol "ACC", URL: https://www.getadcoin.com/
    #[cfg_attr(feature = "serde", serde(rename = "ACC"))]
    Adcoin,

    /// Bridgecoin, symbol "BCO", URL: https://bridgecoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "BCO"))]
    Bridgecoin,

    /// Ellaism, symbol "ELLA", URL: https://ellaism.org
    #[cfg_attr(feature = "serde", serde(rename = "ELLA"))]
    Ellaism,

    /// Pirl, symbol "PIRL", URL: https://pirl.io
    #[cfg_attr(feature = "serde", serde(rename = "PIRL"))]
    Pirl,

    /// RaiBlocks, symbol "XRB", URL: https://raiblocks.com
    #[cfg_attr(feature = "serde", serde(rename = "XRB"))]
    RaiBlocks,

    /// Vivo, symbol "VIVO", URL: https://www.vivocrypto.com/
    #[cfg_attr(feature = "serde", serde(rename = "VIVO"))]
    Vivo,

    /// Firstcoin, symbol "FRST", URL: http://firstcoinproject.com
    #[cfg_attr(feature = "serde", serde(rename = "FRST"))]
    Firstcoin,

    /// Helleniccoin, symbol "HNC", URL: http://www.helleniccoin.gr/
    #[cfg_attr(feature = "serde", serde(rename = "HNC"))]
    Helleniccoin,

    /// BUZZ, symbol "BUZZ", URL: http://www.buzzcoin.info/
    #[cfg_attr(feature = "serde", serde(rename = "BUZZ"))]
    BUZZ,

    /// Ember, symbol "MBRS", URL: https://www.embercoin.io/
    #[cfg_attr(feature = "serde", serde(rename = "MBRS"))]
    Ember,

    /// Hcash, symbol "HSR", URL: https://h.cash
    #[cfg_attr(feature = "serde", serde(rename = "HSR"))]
    Hcash,

    /// HTMLCOIN, symbol "HTML", URL: https://htmlcoin.com/
    #[cfg_attr(feature = "serde", serde(rename = "HTML"))]
    HTMLCOIN,

    /// Obsidian, symbol "ODN", URL: https://obsidianplatform.com/
    #[cfg_attr(feature = "serde", serde(rename = "ODN"))]
    Obsidian,

    /// OnixCoin, symbol "ONX", URL: https://www.onixcoin.com/
    #[cfg_attr(feature = "serde", serde(rename = "ONX"))]
    OnixCoin,

    /// Ravencoin, symbol "RVN", URL: https://ravencoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "RVN"))]
    Ravencoin,

    /// GoByte, symbol "GBX", URL: https://gobyte.network
    #[cfg_attr(feature = "serde", serde(rename = "GBX"))]
    GoByte,

    /// BitcoinZ, symbol "BTCZ", URL: https://btcz.rocks/en/
    #[cfg_attr(feature = "serde", serde(rename = "BTCZ"))]
    BitcoinZ,

    /// Poa, symbol "POA", URL: https://poa.network
    #[cfg_attr(feature = "serde", serde(rename = "POA"))]
    Poa,

    /// NewYorkCoin, symbol "NYC", URL: http://nycoin.net
    #[cfg_attr(feature = "serde", serde(rename = "NYC"))]
    NewYorkCoin,

    /// MarteXcoin, symbol "MXT", URL: http://martexcoin.org
    #[cfg_attr(feature = "serde", serde(rename = "MXT"))]
    MarteXcoin,

    /// Wincoin, symbol "WC", URL: https://wincoin.co
    #[cfg_attr(feature = "serde", serde(rename = "WC"))]
    Wincoin,

    /// Minexcoin, symbol "MNX", URL: https://minexcoin.com
    #[cfg_attr(feature = "serde", serde(rename = "MNX"))]
    Minexcoin,

    /// Bitcoin Private, symbol "BTCP", URL: https://btcprivate.org
    #[cfg_attr(feature = "serde", serde(rename = "BTCP"))]
    BitcoinPrivate,

    /// Musicoin, symbol "MUSIC", URL: https://www.musicoin.org
    #[cfg_attr(feature = "serde", serde(rename = "MUSIC"))]
    Musicoin,

    /// World Bitcoin, symbol "WBTC", URL: http://www.wbtcteam.org/
    #[cfg_attr(feature = "serde", serde(rename = "WBTC"))]
    WorldBitcoin,

    /// Omni, symbol "OMNI", URL: http://www.omnilayer.org
    #[cfg_attr(feature = "serde", serde(rename = "OMNI"))]
    Omni,

    /// BoxyCoin, symbol "BOXY", URL: http://www.boxycoin.org/
    #[cfg_attr(feature = "serde", serde(rename = "BOXY"))]
    BoxyCoin,

    /// Bitcoin Green, symbol "BITG", URL: https://savebitcoin.io
    #[cfg_attr(feature = "serde", serde(rename = "BITG"))]
    BitcoinGreen,

    /// AskCoin, symbol "ASK", URL: https://askcoin.org
    #[cfg_attr(feature = "serde", serde(rename = "ASK"))]
    AskCoin,

    /// Smartcash, symbol "SMART", URL: https://smartcash.cc
    #[cfg_attr(feature = "serde", serde(rename = "SMART"))]
    Smartcash,

    /// XUEZ, symbol "XUEZ", URL: https://xuezcoin.com
    #[cfg_attr(feature = "serde", serde(rename = "XUEZ"))]
    XUEZ,

    /// Varda, symbol "VAR", URL: https://varda.io
    #[cfg_attr(feature = "serde", serde(rename = "VAR"))]
    Varda,

    /// Bitcoin Nano, symbol "NANO", URL: https://www.btcnano.org
    #[cfg_attr(feature = "serde", serde(rename = "NANO"))]
    BitcoinNano,

    /// Blocknet, symbol "BLOCK", URL: https://blocknet.co/
    #[cfg_attr(feature = "serde", serde(rename = "BLOCK"))]
    Blocknet,

    /// MemCoin, symbol "MEM", URL: https://memcoin.org
    #[cfg_attr(feature = "serde", serde(rename = "MEM"))]
    MemCoin,

    /// Phore, symbol "PHR", URL: https://phore.io
    #[cfg_attr(feature = "serde", serde(rename = "PHR"))]
    Phore,

    /// Koto, symbol "KOTO", URL: https://koto.cash/
    #[cfg_attr(feature = "serde", serde(rename = "KOTO"))]
    Koto,

    /// Radiant, symbol "XRD", URL: https://radiant.cash/
    #[cfg_attr(feature = "serde", serde(rename = "XRD"))]
    Radiant,

    /// Bitcoin Smart, symbol "BCS", URL: http://bcs.info
    #[cfg_attr(feature = "serde", serde(rename = "BCS"))]
    BitcoinSmart,

    /// Achain, symbol "ACT", URL: https://www.achain.com/
    #[cfg_attr(feature = "serde", serde(rename = "ACT"))]
    Achain,

    /// Bitcoin World, symbol "BTW", URL: http://btw.one
    #[cfg_attr(feature = "serde", serde(rename = "BTW"))]
    BitcoinWorld,

    /// NEO, symbol "NEO", URL: https://neo.org/
    #[cfg_attr(feature = "serde", serde(rename = "NEO"))]
    NEO,

    /// Bitcoin Diamond, symbol "BCD", URL: http://btcd.io/
    #[cfg_attr(feature = "serde", serde(rename = "BCD"))]
    BitcoinDiamond,

    /// Bitcoin New, symbol "BTN", URL: http://bitcoinnew.org/
    #[cfg_attr(feature = "serde", serde(rename = "BTN"))]
    BitcoinNew,

    /// Big Bitcoin, symbol "BBC", URL: http://bigbitcoins.org/
    #[cfg_attr(feature = "serde", serde(rename = "BBC"))]
    BigBitcoin,

    /// Bitcoin Candy, symbol "CDY", URL: http://www.bitcoincandy.one
    #[cfg_attr(feature = "serde", serde(rename = "CDY"))]
    BitcoinCandy,

    /// Defcoin, symbol "DFC", URL: http://defcoin-ng.org
    #[cfg_attr(feature = "serde", serde(rename = "DFC"))]
    Defcoin,

    /// Cardano, symbol "ADA", URL: https://www.cardanohub.org/en/home/
    #[cfg_attr(feature = "serde", serde(rename = "ADA"))]
    Cardano,

    /// HOdlcoin, symbol "HODL", URL: https://hodlcoin.com/
    #[cfg_attr(feature = "serde", serde(rename = "HODL"))]
    HOdlcoin,

    /// Axe, symbol "AXE", URL: https://github.com/AXErunners/axe
    #[cfg_attr(feature = "serde", serde(rename = "AXE"))]
    Axe,

    /// Bitcoin Pizza, symbol "BPA", URL: http://p.top/
    #[cfg_attr(feature = "serde", serde(rename = "BPA"))]
    BitcoinPizza,

    /// BitcoinQuark, symbol "BTQ", URL: https://www.bitcoinquark.org
    #[cfg_attr(feature = "serde", serde(rename = "BTQ"))]
    BitcoinQuark,

    /// Super Bitcoin, symbol "SBTC", URL: https://www.superbtc.org
    #[cfg_attr(feature = "serde", serde(rename = "SBTC"))]
    SuperBitcoin,

    /// Bitcoin Pay, symbol "BTP", URL: http://www.btceasypay.com
    #[cfg_attr(feature = "serde", serde(rename = "BTP"))]
    BitcoinPay,

    /// Bitcoin Faith, symbol "BTF", URL: http://bitcoinfaith.org
    #[cfg_attr(feature = "serde", serde(rename = "BTF"))]
    BitcoinFaith,

    /// Bitvote, symbol "BTV", URL: www.bitvote.one
    #[cfg_attr(feature = "serde", serde(rename = "BTV"))]
    Bitvote,

    /// Wanchain, symbol "WAN", URL: https://wanchain.org/
    #[cfg_attr(feature = "serde", serde(rename = "WAN"))]
    Wanchain,

    /// Waves, symbol "WAVES", URL: https://wavesplatform.com/
    #[cfg_attr(feature = "serde", serde(rename = "WAVES"))]
    Waves,

    #[doc(hidden)] __Nonexhaustive,
}

impl Coin {
    /// Converts a coin symbol (e.g. `"BTC"`) into its appropriate enum value.
    /// Supports both fully lower-case and fully upper-case variants, but no
    /// mixed-case symbols.
    ///
    /// ```
    /// # extern crate wallet_gen;
    /// # use wallet_gen::coin::Coin;
    /// # fn main() {
    /// assert_eq!(Coin::from_symbol("LTC"), Some(Coin::Litecoin));
    /// assert_eq!(Coin::from_symbol("ltc"), Some(Coin::Litecoin));
    /// assert_eq!(Coin::from_symbol("Ltc"), None);
    /// assert_eq!(Coin::from_symbol("???"), None);
    /// # }
    /// ```
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "btc" | "BTC" => Some(Bitcoin),
            "test" | "TEST" => Some(Testnet),
            "ltc" | "LTC" => Some(Litecoin),
            "doge" | "DOGE" => Some(Dogecoin),
            "rdd" | "RDD" => Some(Reddcoin),
            "dsh" | "DSH" => Some(Dash),
            "ppc" | "PPC" => Some(Peercoin),
            "nmc" | "NMC" => Some(Namecoin),
            "ftc" | "FTC" => Some(Feathercoin),
            "xcp" | "XCP" => Some(Counterparty),
            "blk" | "BLK" => Some(Blackcoin),
            "nsr" | "NSR" => Some(NuShares),
            "nbt" | "NBT" => Some(NuBits),
            "mzc" | "MZC" => Some(Mazacoin),
            "via" | "VIA" => Some(Viacoin),
            "xch" | "XCH" => Some(ClearingHouse),
            "rby" | "RBY" => Some(Rubycoin),
            "grs" | "GRS" => Some(Groestlcoin),
            "dgc" | "DGC" => Some(Digitalcoin),
            "ccn" | "CCN" => Some(Cannacoin),
            "dgb" | "DGB" => Some(DigiByte),
            "mona" | "MONA" => Some(Monacoin),
            "clam" | "CLAM" => Some(Clams),
            "xpm" | "XPM" => Some(Primecoin),
            "neos" | "NEOS" => Some(Neoscoin),
            "jbs" | "JBS" => Some(Jumbucks),
            "zrc" | "ZRC" => Some(ZiftCoin),
            "vtc" | "VTC" => Some(Vertcoin),
            "nxt" | "NXT" => Some(NXT),
            "burst" | "BURST" => Some(Burst),
            "mue" | "MUE" => Some(MonetaryUnit),
            "zoom" | "ZOOM" => Some(Zoom),
            "vpn" | "VPN" => Some(Vpncoin),
            "cdn" | "CDN" => Some(CanadaeCoin),
            "sdc" | "SDC" => Some(ShadowCash),
            "pkb" | "PKB" => Some(ParkByte),
            "pnd" | "PND" => Some(Pandacoin),
            "start" | "START" => Some(StartCoin),
            "moin" | "MOIN" => Some(MOIN),
            "exp" | "EXP" => Some(Expanse),
            "dcr" | "DCR" => Some(Decred),
            "xem" | "XEM" => Some(NEM),
            "part" | "PART" => Some(Particl),
            "arg" | "ARG" => Some(Argentum),
            "shr" | "SHR" => Some(Shreeji),
            "gcr" | "GCR" => Some(GcrCoin),
            "nvc" | "NVC" => Some(Novacoin),
            "ac" | "AC" => Some(Asiacoin),
            "btcd" | "BTCD" => Some(Bitcoindark),
            "dope" | "DOPE" => Some(Dopecoin),
            "tpc" | "TPC" => Some(Templecoin),
            "aib" | "AIB" => Some(AIB),
            "edrc" | "EDRC" => Some(EDRCoin),
            "sys" | "SYS" => Some(Syscoin),
            "slr" | "SLR" => Some(Solarcoin),
            "smly" | "SMLY" => Some(Smileycoin),
            "eth" | "ETH" => Some(Ethereum),
            "etc" | "ETC" => Some(EthereumClassic),
            "psb" | "PSB" => Some(Pesobit),
            "ldcn" | "LDCN" => Some(Landcoin),
            "xbc" | "XBC" => Some(Bitcoinplus),
            "iop" | "IOP" => Some(InternetofPeople),
            "nxs" | "NXS" => Some(Nexus),
            "insn" | "INSN" => Some(InsaneCoin),
            "ok" | "OK" => Some(OkCash),
            "brit" | "BRIT" => Some(BritCoin),
            "cmp" | "CMP" => Some(Compcoin),
            "crw" | "CRW" => Some(Crown),
            "bela" | "BELA" => Some(BelaCoin),
            "vash" | "VASH" => Some(VirtualCash),
            "fjc" | "FJC" => Some(FujiCoin),
            "mix" | "MIX" => Some(MIX),
            "xvg" | "XVG" => Some(Verge),
            "efl" | "EFL" => Some(ElectronicGulden),
            "club" | "CLUB" => Some(ClubCoin),
            "richx" | "RICHX" => Some(RichCoin),
            "pot" | "POT" => Some(Potcoin),
            "qrk" | "QRK" => Some(Quarkcoin),
            "trc" | "TRC" => Some(Terracoin),
            "grc" | "GRC" => Some(Gridcoin),
            "aur" | "AUR" => Some(Auroracoin),
            "ixc" | "IXC" => Some(IXCoin),
            "nlg" | "NLG" => Some(Gulden),
            "bitb" | "BITB" => Some(BitBean),
            "bta" | "BTA" => Some(Bata),
            "xmy" | "XMY" => Some(Myriadcoin),
            "bsd" | "BSD" => Some(BitSend),
            "uno" | "UNO" => Some(Unobtanium),
            "mtr" | "MTR" => Some(MasterTrader),
            "gb" | "GB" => Some(GoldBlocks),
            "shm" | "SHM" => Some(Saham),
            "crx" | "CRX" => Some(Chronos),
            "biq" | "BIQ" => Some(Ubiquoin),
            "evo" | "EVO" => Some(Evotion),
            "sto" | "STO" => Some(SaveTheOcean),
            "bigup" | "BIGUP" => Some(BigUp),
            "gmc" | "GMC" => Some(GameCredits),
            "dlc" | "DLC" => Some(Dollarcoins),
            "zyd" | "ZYD" => Some(Zayedcoin),
            "dbic" | "DBIC" => Some(Dubaicoin),
            "strat" | "STRAT" => Some(Stratis),
            "sh" | "SH" => Some(Shilling),
            "mars" | "MARS" => Some(MarsCoin),
            "ubq" | "UBQ" => Some(Ubiq),
            "ptc" | "PTC" => Some(Pesetacoin),
            "nrc" | "NRC" => Some(Neurocoin),
            "ark" | "ARK" => Some(ARK),
            "usc" | "USC" => Some(UltimateSecureCashMain),
            "hmp" | "HMP" => Some(Hempcoin),
            "linx" | "LINX" => Some(Linx),
            "ecn" | "ECN" => Some(Ecoin),
            "dnr" | "DNR" => Some(Denarius),
            "pink" | "PINK" => Some(Pinkcoin),
            "piggy" | "PIGGY" => Some(PiggyCoin),
            "pivx" | "PIVX" => Some(Pivx),
            "flash" | "FLASH" => Some(Flashcoin),
            "zen" | "ZEN" => Some(Zencash),
            "put" | "PUT" => Some(Putincoin),
            "zny" | "ZNY" => Some(BitZeny),
            "unify" | "UNIFY" => Some(Unify),
            "xst" | "XST" => Some(StealthCoin),
            "brk" | "BRK" => Some(BreakoutCoin),
            "vc" | "VC" => Some(Vcash),
            "xmr" | "XMR" => Some(Monero),
            "vox" | "VOX" => Some(Voxels),
            "nav" | "NAV" => Some(NavCoin),
            "fct" | "FCT" => Some(FactomFactoids),
            "ec" | "EC" => Some(FactomEntryCredits),
            "zec" | "ZEC" => Some(Zcash),
            "lsk" | "LSK" => Some(Lisk),
            "steem" | "STEEM" => Some(Steem),
            "xzc" | "XZC" => Some(ZCoin),
            "rsk" | "RSK" => Some(Rootstock),
            "rpt" | "RPT" => Some(RealPointCoin),
            "lbc" | "LBC" => Some(LBRYCredits),
            "kmd" | "KMD" => Some(Komodo),
            "bsq" | "BSQ" => Some(BisqToken),
            "ric" | "RIC" => Some(Riecoin),
            "xrp" | "XRP" => Some(Ripple),
            "bch" | "BCH" => Some(BitcoinCash),
            "nebl" | "NEBL" => Some(Neblio),
            "zcl" | "ZCL" => Some(ZClassic),
            "xlm" | "XLM" => Some(StellarLumens),
            "whl" | "WHL" => Some(WhaleCoin),
            "erc" | "ERC" => Some(EuropeCoin),
            "dmd" | "DMD" => Some(Diamond),
            "btm" | "BTM" => Some(Bytom),
            "bio" | "BIO" => Some(Biocoin),
            "xwc" | "XWC" => Some(Whitecoin),
            "btg" | "BTG" => Some(BitcoinGold),
            "ssn" | "SSN" => Some(SuperSkynet),
            "toa" | "TOA" => Some(TOACoin),
            "btx" | "BTX" => Some(Bitcore),
            "acc" | "ACC" => Some(Adcoin),
            "bco" | "BCO" => Some(Bridgecoin),
            "ella" | "ELLA" => Some(Ellaism),
            "pirl" | "PIRL" => Some(Pirl),
            "xrb" | "XRB" => Some(RaiBlocks),
            "vivo" | "VIVO" => Some(Vivo),
            "frst" | "FRST" => Some(Firstcoin),
            "hnc" | "HNC" => Some(Helleniccoin),
            "buzz" | "BUZZ" => Some(BUZZ),
            "mbrs" | "MBRS" => Some(Ember),
            "hsr" | "HSR" => Some(Hcash),
            "html" | "HTML" => Some(HTMLCOIN),
            "odn" | "ODN" => Some(Obsidian),
            "onx" | "ONX" => Some(OnixCoin),
            "rvn" | "RVN" => Some(Ravencoin),
            "gbx" | "GBX" => Some(GoByte),
            "btcz" | "BTCZ" => Some(BitcoinZ),
            "poa" | "POA" => Some(Poa),
            "nyc" | "NYC" => Some(NewYorkCoin),
            "mxt" | "MXT" => Some(MarteXcoin),
            "wc" | "WC" => Some(Wincoin),
            "mnx" | "MNX" => Some(Minexcoin),
            "btcp" | "BTCP" => Some(BitcoinPrivate),
            "music" | "MUSIC" => Some(Musicoin),
            "wbtc" | "WBTC" => Some(WorldBitcoin),
            "omni" | "OMNI" => Some(Omni),
            "boxy" | "BOXY" => Some(BoxyCoin),
            "bitg" | "BITG" => Some(BitcoinGreen),
            "ask" | "ASK" => Some(AskCoin),
            "smart" | "SMART" => Some(Smartcash),
            "xuez" | "XUEZ" => Some(XUEZ),
            "var" | "VAR" => Some(Varda),
            "nano" | "NANO" => Some(BitcoinNano),
            "block" | "BLOCK" => Some(Blocknet),
            "mem" | "MEM" => Some(MemCoin),
            "phr" | "PHR" => Some(Phore),
            "koto" | "KOTO" => Some(Koto),
            "xrd" | "XRD" => Some(Radiant),
            "bcs" | "BCS" => Some(BitcoinSmart),
            "act" | "ACT" => Some(Achain),
            "btw" | "BTW" => Some(BitcoinWorld),
            "neo" | "NEO" => Some(NEO),
            "bcd" | "BCD" => Some(BitcoinDiamond),
            "btn" | "BTN" => Some(BitcoinNew),
            "bbc" | "BBC" => Some(BigBitcoin),
            "cdy" | "CDY" => Some(BitcoinCandy),
            "dfc" | "DFC" => Some(Defcoin),
            "ada" | "ADA" => Some(Cardano),
            "hodl" | "HODL" => Some(HOdlcoin),
            "axe" | "AXE" => Some(Axe),
            "bpa" | "BPA" => Some(BitcoinPizza),
            "btq" | "BTQ" => Some(BitcoinQuark),
            "sbtc" | "SBTC" => Some(SuperBitcoin),
            "btp" | "BTP" => Some(BitcoinPay),
            "btf" | "BTF" => Some(BitcoinFaith),
            "btv" | "BTV" => Some(Bitvote),
            "wan" | "WAN" => Some(Wanchain),
            "waves" | "WAVES" => Some(Waves),
            _ => None,
        }
    }

    /// Gets the uppercase coin symbol for an enum value.
    /// This is the opposite of the `from_symbol()` constructor method.
    ///
    /// ```
    /// # extern crate wallet_gen;
    /// # use wallet_gen::coin::Coin;
    /// # fn main() {
    /// let coin = Coin::Ethereum;
    /// assert_eq!(coin.symbol(), "ETH");
    /// assert_eq!(Some(coin), Coin::from_symbol(coin.symbol()));
    /// # }
    /// ```
    pub fn symbol(self) -> &'static str {
        match self {
            Coin::Bitcoin => "BTC",
            Coin::Testnet => "TEST",
            Coin::Litecoin => "LTC",
            Coin::Dogecoin => "DOGE",
            Coin::Reddcoin => "RDD",
            Coin::Dash => "DSH",
            Coin::Peercoin => "PPC",
            Coin::Namecoin => "NMC",
            Coin::Feathercoin => "FTC",
            Coin::Counterparty => "XCP",
            Coin::Blackcoin => "BLK",
            Coin::NuShares => "NSR",
            Coin::NuBits => "NBT",
            Coin::Mazacoin => "MZC",
            Coin::Viacoin => "VIA",
            Coin::ClearingHouse => "XCH",
            Coin::Rubycoin => "RBY",
            Coin::Groestlcoin => "GRS",
            Coin::Digitalcoin => "DGC",
            Coin::Cannacoin => "CCN",
            Coin::DigiByte => "DGB",
            Coin::Monacoin => "MONA",
            Coin::Clams => "CLAM",
            Coin::Primecoin => "XPM",
            Coin::Neoscoin => "NEOS",
            Coin::Jumbucks => "JBS",
            Coin::ZiftCoin => "ZRC",
            Coin::Vertcoin => "VTC",
            Coin::NXT => "NXT",
            Coin::Burst => "BURST",
            Coin::MonetaryUnit => "MUE",
            Coin::Zoom => "ZOOM",
            Coin::Vpncoin => "VPN",
            Coin::CanadaeCoin => "CDN",
            Coin::ShadowCash => "SDC",
            Coin::ParkByte => "PKB",
            Coin::Pandacoin => "PND",
            Coin::StartCoin => "START",
            Coin::MOIN => "MOIN",
            Coin::Expanse => "EXP",
            Coin::Decred => "DCR",
            Coin::NEM => "XEM",
            Coin::Particl => "PART",
            Coin::Argentum => "ARG",
            Coin::Shreeji => "SHR",
            Coin::GcrCoin => "GCR",
            Coin::Novacoin => "NVC",
            Coin::Asiacoin => "AC",
            Coin::Bitcoindark => "BTCD",
            Coin::Dopecoin => "DOPE",
            Coin::Templecoin => "TPC",
            Coin::AIB => "AIB",
            Coin::EDRCoin => "EDRC",
            Coin::Syscoin => "SYS",
            Coin::Solarcoin => "SLR",
            Coin::Smileycoin => "SMLY",
            Coin::Ethereum => "ETH",
            Coin::EthereumClassic => "ETC",
            Coin::Pesobit => "PSB",
            Coin::Landcoin => "LDCN",
            Coin::Bitcoinplus => "XBC",
            Coin::InternetofPeople => "IOP",
            Coin::Nexus => "NXS",
            Coin::InsaneCoin => "INSN",
            Coin::OkCash => "OK",
            Coin::BritCoin => "BRIT",
            Coin::Compcoin => "CMP",
            Coin::Crown => "CRW",
            Coin::BelaCoin => "BELA",
            Coin::VirtualCash => "VASH",
            Coin::FujiCoin => "FJC",
            Coin::MIX => "MIX",
            Coin::Verge => "XVG",
            Coin::ElectronicGulden => "EFL",
            Coin::ClubCoin => "CLUB",
            Coin::RichCoin => "RICHX",
            Coin::Potcoin => "POT",
            Coin::Quarkcoin => "QRK",
            Coin::Terracoin => "TRC",
            Coin::Gridcoin => "GRC",
            Coin::Auroracoin => "AUR",
            Coin::IXCoin => "IXC",
            Coin::Gulden => "NLG",
            Coin::BitBean => "BITB",
            Coin::Bata => "BTA",
            Coin::Myriadcoin => "XMY",
            Coin::BitSend => "BSD",
            Coin::Unobtanium => "UNO",
            Coin::MasterTrader => "MTR",
            Coin::GoldBlocks => "GB",
            Coin::Saham => "SHM",
            Coin::Chronos => "CRX",
            Coin::Ubiquoin => "BIQ",
            Coin::Evotion => "EVO",
            Coin::SaveTheOcean => "STO",
            Coin::BigUp => "BIGUP",
            Coin::GameCredits => "GMC",
            Coin::Dollarcoins => "DLC",
            Coin::Zayedcoin => "ZYD",
            Coin::Dubaicoin => "DBIC",
            Coin::Stratis => "STRAT",
            Coin::Shilling => "SH",
            Coin::MarsCoin => "MARS",
            Coin::Ubiq => "UBQ",
            Coin::Pesetacoin => "PTC",
            Coin::Neurocoin => "NRC",
            Coin::ARK => "ARK",
            Coin::UltimateSecureCashMain => "USC",
            Coin::Hempcoin => "HMP",
            Coin::Linx => "LINX",
            Coin::Ecoin => "ECN",
            Coin::Denarius => "DNR",
            Coin::Pinkcoin => "PINK",
            Coin::PiggyCoin => "PIGGY",
            Coin::Pivx => "PIVX",
            Coin::Flashcoin => "FLASH",
            Coin::Zencash => "ZEN",
            Coin::Putincoin => "PUT",
            Coin::BitZeny => "ZNY",
            Coin::Unify => "UNIFY",
            Coin::StealthCoin => "XST",
            Coin::BreakoutCoin => "BRK",
            Coin::Vcash => "VC",
            Coin::Monero => "XMR",
            Coin::Voxels => "VOX",
            Coin::NavCoin => "NAV",
            Coin::FactomFactoids => "FCT",
            Coin::FactomEntryCredits => "EC",
            Coin::Zcash => "ZEC",
            Coin::Lisk => "LSK",
            Coin::Steem => "STEEM",
            Coin::ZCoin => "XZC",
            Coin::Rootstock => "RSK",
            Coin::RealPointCoin => "RPT",
            Coin::LBRYCredits => "LBC",
            Coin::Komodo => "KMD",
            Coin::BisqToken => "BSQ",
            Coin::Riecoin => "RIC",
            Coin::Ripple => "XRP",
            Coin::BitcoinCash => "BCH",
            Coin::Neblio => "NEBL",
            Coin::ZClassic => "ZCL",
            Coin::StellarLumens => "XLM",
            Coin::WhaleCoin => "WHL",
            Coin::EuropeCoin => "ERC",
            Coin::Diamond => "DMD",
            Coin::Bytom => "BTM",
            Coin::Biocoin => "BIO",
            Coin::Whitecoin => "XWC",
            Coin::BitcoinGold => "BTG",
            Coin::SuperSkynet => "SSN",
            Coin::TOACoin => "TOA",
            Coin::Bitcore => "BTX",
            Coin::Adcoin => "ACC",
            Coin::Bridgecoin => "BCO",
            Coin::Ellaism => "ELLA",
            Coin::Pirl => "PIRL",
            Coin::RaiBlocks => "XRB",
            Coin::Vivo => "VIVO",
            Coin::Firstcoin => "FRST",
            Coin::Helleniccoin => "HNC",
            Coin::BUZZ => "BUZZ",
            Coin::Ember => "MBRS",
            Coin::Hcash => "HSR",
            Coin::HTMLCOIN => "HTML",
            Coin::Obsidian => "ODN",
            Coin::OnixCoin => "ONX",
            Coin::Ravencoin => "RVN",
            Coin::GoByte => "GBX",
            Coin::BitcoinZ => "BTCZ",
            Coin::Poa => "POA",
            Coin::NewYorkCoin => "NYC",
            Coin::MarteXcoin => "MXT",
            Coin::Wincoin => "WC",
            Coin::Minexcoin => "MNX",
            Coin::BitcoinPrivate => "BTCP",
            Coin::Musicoin => "MUSIC",
            Coin::WorldBitcoin => "WBTC",
            Coin::Omni => "OMNI",
            Coin::BoxyCoin => "BOXY",
            Coin::BitcoinGreen => "BITG",
            Coin::AskCoin => "ASK",
            Coin::Smartcash => "SMART",
            Coin::XUEZ => "XUEZ",
            Coin::Varda => "VAR",
            Coin::BitcoinNano => "NANO",
            Coin::Blocknet => "BLOCK",
            Coin::MemCoin => "MEM",
            Coin::Phore => "PHR",
            Coin::Koto => "KOTO",
            Coin::Radiant => "XRD",
            Coin::BitcoinSmart => "BCS",
            Coin::Achain => "ACT",
            Coin::BitcoinWorld => "BTW",
            Coin::NEO => "NEO",
            Coin::BitcoinDiamond => "BCD",
            Coin::BitcoinNew => "BTN",
            Coin::BigBitcoin => "BBC",
            Coin::BitcoinCandy => "CDY",
            Coin::Defcoin => "DFC",
            Coin::Cardano => "ADA",
            Coin::HOdlcoin => "HODL",
            Coin::Axe => "AXE",
            Coin::BitcoinPizza => "BPA",
            Coin::BitcoinQuark => "BTQ",
            Coin::SuperBitcoin => "SBTC",
            Coin::BitcoinPay => "BTP",
            Coin::BitcoinFaith => "BTF",
            Coin::Bitvote => "BTV",
            Coin::Wanchain => "WAN",
            Coin::Waves => "WAVES",
            Coin::__Nonexhaustive => unreachable!(),
        }
    }

    /// Gets the constant data used to generate WIF strings
    /// in Bitcoin-style wallets.
    pub fn bitcoin_wif_data(self) -> Option<BitcoinWifData> {
        Some(match self {
            Coin::Bitcoin => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
            Coin::Testnet => BitcoinWifData(&[0x6f], 0xef, "9", "c"),
            Coin::Litecoin => BitcoinWifData(&[0x30], 0xb0, "6", "T"),
            Coin::Dogecoin => BitcoinWifData(&[0x1e], 0x9e, "6", "Q"),
            Coin::Reddcoin => BitcoinWifData(&[0x3d], 0xbd, "7", "UV"),
            Coin::Dash => BitcoinWifData(&[0x4c], 0xcc, "7", "X"),
            Coin::Peercoin => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
            Coin::Namecoin => BitcoinWifData(&[0x34], 0x80, "5", "LK"),
            Coin::Feathercoin => BitcoinWifData(&[0x0e], 0x8e, "5", "N"),
            Coin::Blackcoin => BitcoinWifData(&[0x19], 0x99, "6", "P"),
            Coin::NuBits => BitcoinWifData(&[0x19], 0xbf, "7", "V"),
            Coin::Mazacoin => BitcoinWifData(&[0x32], 0xe0, "8", "a"),
            Coin::Viacoin => BitcoinWifData(&[0x47], 0xc7, "7", "W"),
            Coin::Rubycoin => BitcoinWifData(&[0x3c], 0xbc, "7", "U"),
            Coin::Digitalcoin => BitcoinWifData(&[0x1e], 0x9e, "6", "Q"),
            Coin::Cannacoin => BitcoinWifData(&[0x1c], 0x9c, "6", "Q"),
            Coin::DigiByte => BitcoinWifData(&[0x1e], 0x9e, "6", "Q"),
            Coin::Primecoin => BitcoinWifData(&[0x17], 0x97, "6", "P"),
            Coin::Neoscoin => BitcoinWifData(&[0x35], 0xb1, "6", "T"),
            Coin::Jumbucks => BitcoinWifData(&[0x2b], 0xab, "6", "S"),
            Coin::Vertcoin => BitcoinWifData(&[0x47], 0x80, "5", "LK"),
            Coin::MonetaryUnit => BitcoinWifData(&[0x10], 0x7e, "5", "K"),
            Coin::CanadaeCoin => BitcoinWifData(&[0x1c], 0x9c, "6", "Q"),
            Coin::ParkByte => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
            Coin::Pandacoin => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
            Coin::Particl => BitcoinWifData(&[0x38], 0x6c, "4", "HG"),
            Coin::Novacoin => BitcoinWifData(&[0x08], 0x88, "5", "M"),
            Coin::Bitcoindark => BitcoinWifData(&[0x3c], 0xbc, "7", "U"),
            Coin::Syscoin => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
            Coin::Smileycoin => BitcoinWifData(&[0x19], 0x99, "6", "P"),
            Coin::FujiCoin => BitcoinWifData(&[0x24], 0xa4, "6", "R"),
            Coin::ElectronicGulden => BitcoinWifData(&[0x30], 0xb0, "6", "T"),
            Coin::Potcoin => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
            Coin::Quarkcoin => BitcoinWifData(&[0x3a], 0xba, "7", "U"),
            Coin::Terracoin => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
            Coin::Gridcoin => BitcoinWifData(&[0x3e], 0xbe, "7", "V"),
            Coin::Auroracoin => BitcoinWifData(&[0x17], 0x97, "6", "T"),
            Coin::Gulden => BitcoinWifData(&[0x26], 0xa6, "6", "R"),
            Coin::Myriadcoin => BitcoinWifData(&[0x32], 0xb2, "6", "T"),
            Coin::Unobtanium => BitcoinWifData(&[0x82], 0xe0, "8", "a"),
            Coin::Stratis => BitcoinWifData(&[0x3f], 0xbf, "7", "V"),
            Coin::MarsCoin => BitcoinWifData(&[0x32], 0xb2, "6", "T"),
            Coin::Pesetacoin => BitcoinWifData(&[0x2f], 0xaf, "6", "ST"),
            Coin::Pinkcoin => BitcoinWifData(&[0x03], 0x83, "RQP", "L"),
            Coin::PiggyCoin => BitcoinWifData(&[0x76], 0xf6, "9", "d"),
            Coin::Pivx => BitcoinWifData(&[0x1e], 0xd4, "8", "Y"),
            Coin::BitZeny => BitcoinWifData(&[0x51], 0x80, "5", "LK"),
            Coin::StealthCoin => BitcoinWifData(&[0x3e], 0xbe, "7", "V"),
            Coin::Vcash => BitcoinWifData(&[0x47], 0xc7, "7", "W"),
            Coin::NavCoin => BitcoinWifData(&[0x35], 0x96, "6", "P"),
            Coin::Zcash => BitcoinWifData(&[0x1c, 0xb8], 0x80, "5", "LK"),
            Coin::LBRYCredits => BitcoinWifData(&[0x55], 0x80, "5", "LK"),
            Coin::Riecoin => BitcoinWifData(&[0x3c], 0x80, "5", "LK"),
            Coin::BitcoinCash => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
            Coin::BitcoinGold => BitcoinWifData(&[0x26], 0x80, "5", "LK"),
            Coin::Bitcore => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
            Coin::Ember => BitcoinWifData(&[0x5c], 0x32, "2", "8"),
            Coin::HTMLCOIN => BitcoinWifData(&[0x29], 0xa9, "6", "S"),
            Coin::MarteXcoin => BitcoinWifData(&[0x32], 0xb2, "6", "T"),
            Coin::Omni => BitcoinWifData(&[0x73], 0xf3, "9", "cd"),
            Coin::BoxyCoin => BitcoinWifData(&[0x4b], 0xcb, "7", "X"),
            Coin::Blocknet => BitcoinWifData(&[0x1a], 0x9a, "6", "P"),
            Coin::HOdlcoin => BitcoinWifData(&[0x28], 0xa8, "5", "LK"),
            Coin::Axe => BitcoinWifData(&[0x4b], 0xcb, "7", "X"),
            Coin::__Nonexhaustive => unreachable!(),
            _ => return None,
        })
    }
}

/// List of all [`Coin`]s in the enumeration, in altcoin index order.
///
/// [`Coin`] ./enum.Coin.html
pub const COINS: [Coin; 211] = [
    Coin::Bitcoin,
    Coin::Testnet,
    Coin::Litecoin,
    Coin::Dogecoin,
    Coin::Reddcoin,
    Coin::Dash,
    Coin::Peercoin,
    Coin::Namecoin,
    Coin::Feathercoin,
    Coin::Counterparty,
    Coin::Blackcoin,
    Coin::NuShares,
    Coin::NuBits,
    Coin::Mazacoin,
    Coin::Viacoin,
    Coin::ClearingHouse,
    Coin::Rubycoin,
    Coin::Groestlcoin,
    Coin::Digitalcoin,
    Coin::Cannacoin,
    Coin::DigiByte,
    Coin::Monacoin,
    Coin::Clams,
    Coin::Primecoin,
    Coin::Neoscoin,
    Coin::Jumbucks,
    Coin::ZiftCoin,
    Coin::Vertcoin,
    Coin::NXT,
    Coin::Burst,
    Coin::MonetaryUnit,
    Coin::Zoom,
    Coin::Vpncoin,
    Coin::CanadaeCoin,
    Coin::ShadowCash,
    Coin::ParkByte,
    Coin::Pandacoin,
    Coin::StartCoin,
    Coin::MOIN,
    Coin::Expanse,
    Coin::Decred,
    Coin::NEM,
    Coin::Particl,
    Coin::Argentum,
    Coin::Shreeji,
    Coin::GcrCoin,
    Coin::Novacoin,
    Coin::Asiacoin,
    Coin::Bitcoindark,
    Coin::Dopecoin,
    Coin::Templecoin,
    Coin::AIB,
    Coin::EDRCoin,
    Coin::Syscoin,
    Coin::Solarcoin,
    Coin::Smileycoin,
    Coin::Ethereum,
    Coin::EthereumClassic,
    Coin::Pesobit,
    Coin::Landcoin,
    Coin::Bitcoinplus,
    Coin::InternetofPeople,
    Coin::Nexus,
    Coin::InsaneCoin,
    Coin::OkCash,
    Coin::BritCoin,
    Coin::Compcoin,
    Coin::Crown,
    Coin::BelaCoin,
    Coin::VirtualCash,
    Coin::FujiCoin,
    Coin::MIX,
    Coin::Verge,
    Coin::ElectronicGulden,
    Coin::ClubCoin,
    Coin::RichCoin,
    Coin::Potcoin,
    Coin::Quarkcoin,
    Coin::Terracoin,
    Coin::Gridcoin,
    Coin::Auroracoin,
    Coin::IXCoin,
    Coin::Gulden,
    Coin::BitBean,
    Coin::Bata,
    Coin::Myriadcoin,
    Coin::BitSend,
    Coin::Unobtanium,
    Coin::MasterTrader,
    Coin::GoldBlocks,
    Coin::Saham,
    Coin::Chronos,
    Coin::Ubiquoin,
    Coin::Evotion,
    Coin::SaveTheOcean,
    Coin::BigUp,
    Coin::GameCredits,
    Coin::Dollarcoins,
    Coin::Zayedcoin,
    Coin::Dubaicoin,
    Coin::Stratis,
    Coin::Shilling,
    Coin::MarsCoin,
    Coin::Ubiq,
    Coin::Pesetacoin,
    Coin::Neurocoin,
    Coin::ARK,
    Coin::UltimateSecureCashMain,
    Coin::Hempcoin,
    Coin::Linx,
    Coin::Ecoin,
    Coin::Denarius,
    Coin::Pinkcoin,
    Coin::PiggyCoin,
    Coin::Pivx,
    Coin::Flashcoin,
    Coin::Zencash,
    Coin::Putincoin,
    Coin::BitZeny,
    Coin::Unify,
    Coin::StealthCoin,
    Coin::BreakoutCoin,
    Coin::Vcash,
    Coin::Monero,
    Coin::Voxels,
    Coin::NavCoin,
    Coin::FactomFactoids,
    Coin::FactomEntryCredits,
    Coin::Zcash,
    Coin::Lisk,
    Coin::Steem,
    Coin::ZCoin,
    Coin::Rootstock,
    Coin::RealPointCoin,
    Coin::LBRYCredits,
    Coin::Komodo,
    Coin::BisqToken,
    Coin::Riecoin,
    Coin::Ripple,
    Coin::BitcoinCash,
    Coin::Neblio,
    Coin::ZClassic,
    Coin::StellarLumens,
    Coin::WhaleCoin,
    Coin::EuropeCoin,
    Coin::Diamond,
    Coin::Bytom,
    Coin::Biocoin,
    Coin::Whitecoin,
    Coin::BitcoinGold,
    Coin::SuperSkynet,
    Coin::TOACoin,
    Coin::Bitcore,
    Coin::Adcoin,
    Coin::Bridgecoin,
    Coin::Ellaism,
    Coin::Pirl,
    Coin::RaiBlocks,
    Coin::Vivo,
    Coin::Firstcoin,
    Coin::Helleniccoin,
    Coin::BUZZ,
    Coin::Ember,
    Coin::Hcash,
    Coin::HTMLCOIN,
    Coin::Obsidian,
    Coin::OnixCoin,
    Coin::Ravencoin,
    Coin::GoByte,
    Coin::BitcoinZ,
    Coin::Poa,
    Coin::NewYorkCoin,
    Coin::MarteXcoin,
    Coin::Wincoin,
    Coin::Minexcoin,
    Coin::BitcoinPrivate,
    Coin::Musicoin,
    Coin::WorldBitcoin,
    Coin::Omni,
    Coin::BoxyCoin,
    Coin::BitcoinGreen,
    Coin::AskCoin,
    Coin::Smartcash,
    Coin::XUEZ,
    Coin::Varda,
    Coin::BitcoinNano,
    Coin::Blocknet,
    Coin::MemCoin,
    Coin::Phore,
    Coin::Koto,
    Coin::Radiant,
    Coin::BitcoinSmart,
    Coin::Achain,
    Coin::BitcoinWorld,
    Coin::NEO,
    Coin::BitcoinDiamond,
    Coin::BitcoinNew,
    Coin::BigBitcoin,
    Coin::BitcoinCandy,
    Coin::Defcoin,
    Coin::Cardano,
    Coin::HOdlcoin,
    Coin::Axe,
    Coin::BitcoinPizza,
    Coin::BitcoinQuark,
    Coin::SuperBitcoin,
    Coin::BitcoinPay,
    Coin::BitcoinFaith,
    Coin::Bitvote,
    Coin::Wanchain,
    Coin::Waves,
];

/// Constants for generating addresses and private keys in Bitcoin-based
/// coins, when converting to Wallet Import Format (WIF).
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BitcoinWifData(&'static [u8], u8, &'static str, &'static str);

impl BitcoinWifData {
    /// Gets the network version used in WIF conversion.
    #[inline]
    pub fn network_version(self) -> &'static [u8] { self.0 }

    /// Gets the prefix to prepend when creating the WIF private key.
    #[inline]
    pub fn private_key_prefix(self) -> u8 { self.1 }

    /// Returns the WIF character set for the beginning of the string.
    #[inline]
    pub fn wif_start(self) -> &'static str { self.2 }

    /// Returns the CWIF character set for the beginning of the string.
    #[inline]
    pub fn cwif_start(self) -> &'static str { self.3 }

    /// See if the given string matches the character set returned by `wif_start()`.
    pub fn check_wif(self, s: &str) -> bool {
        for c in self.wif_start().chars() {
            if s.starts_with(c) {
                return true;
            }
        }

        false
    }

    /// See if the given string matches the character set returned by `cwif_start()`.
    pub fn check_cwif(self, s: &str) -> bool {
        for c in self.cwif_start().chars() {
            if s.starts_with(c) {
                return true;
            }
        }

        false
    }
}
