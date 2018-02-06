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
    CanadaEcoin,

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
            "cdn" | "CDN" => Some(CanadaEcoin),
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
            Coin::CanadaEcoin => "CDN",
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

    /// Gets the coin hexadecimal value for
    /// [registered coin types](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    pub fn hexa(self) -> u32 {
        match self {
            Coin::Bitcoin => 0x80000000,
            Coin::Testnet => 0x00000000,
            Coin::Litecoin => 0x80000002,
            Coin::Dogecoin => 0x80000003,
            Coin::Reddcoin => 0x80000004,
            Coin::Dash => 0x80000005,
            Coin::Peercoin => 0x80000006,
            Coin::Namecoin => 0x80000007,
            Coin::Feathercoin => 0x80000008,
            Coin::Counterparty => 0x80000009,
            Coin::Blackcoin => 0x8000000a,
            Coin::NuShares => 0x8000000b,
            Coin::NuBits => 0x8000000c,
            Coin::Mazacoin => 0x8000000d,
            Coin::Viacoin => 0x8000000e,
            Coin::ClearingHouse => 0x8000000f,
            Coin::Rubycoin => 0x80000010,
            Coin::Groestlcoin => 0x80000011,
            Coin::Digitalcoin => 0x80000012,
            Coin::Cannacoin => 0x80000013,
            Coin::DigiByte => 0x80000014,
            Coin::Monacoin => 0x80000016,
            Coin::Clams => 0x80000017,
            Coin::Primecoin => 0x80000018,
            Coin::Neoscoin => 0x80000019,
            Coin::Jumbucks => 0x8000001a,
            Coin::ZiftCoin => 0x8000001b,
            Coin::Vertcoin => 0x8000001c,
            Coin::NXT => 0x8000001d,
            Coin::Burst => 0x8000001e,
            Coin::MonetaryUnit => 0x8000001f,
            Coin::Zoom => 0x80000020,
            Coin::Vpncoin => 0x80000021,
            Coin::CanadaEcoin => 0x80000022,
            Coin::ShadowCash => 0x80000023,
            Coin::ParkByte => 0x80000024,
            Coin::Pandacoin => 0x80000025,
            Coin::StartCoin => 0x80000026,
            Coin::MOIN => 0x80000027,
            Coin::Expanse => 0x80000028,
            Coin::Decred => 0x8000002a,
            Coin::NEM => 0x8000002b,
            Coin::Particl => 0x8000002c,
            Coin::Argentum => 0x8000002d,
            Coin::Shreeji => 0x80000030,
            Coin::GcrCoin => 0x80000031,
            Coin::Novacoin => 0x80000032,
            Coin::Asiacoin => 0x80000033,
            Coin::Bitcoindark => 0x80000034,
            Coin::Dopecoin => 0x80000035,
            Coin::Templecoin => 0x80000036,
            Coin::AIB => 0x80000037,
            Coin::EDRCoin => 0x80000038,
            Coin::Syscoin => 0x80000039,
            Coin::Solarcoin => 0x8000003a,
            Coin::Smileycoin => 0x8000003b,
            Coin::Ethereum => 0x8000003c,
            Coin::EthereumClassic => 0x8000003d,
            Coin::Pesobit => 0x8000003e,
            Coin::Landcoin => 0x8000003f,
            Coin::Bitcoinplus => 0x80000041,
            Coin::InternetofPeople => 0x80000042,
            Coin::Nexus => 0x80000043,
            Coin::InsaneCoin => 0x80000044,
            Coin::OkCash => 0x80000045,
            Coin::BritCoin => 0x80000046,
            Coin::Compcoin => 0x80000047,
            Coin::Crown => 0x80000048,
            Coin::BelaCoin => 0x80000049,
            Coin::VirtualCash => 0x8000004a,
            Coin::FujiCoin => 0x8000004b,
            Coin::MIX => 0x8000004c,
            Coin::Verge => 0x8000004d,
            Coin::ElectronicGulden => 0x8000004e,
            Coin::ClubCoin => 0x8000004f,
            Coin::RichCoin => 0x80000050,
            Coin::Potcoin => 0x80000051,
            Coin::Quarkcoin => 0x80000052,
            Coin::Terracoin => 0x80000053,
            Coin::Gridcoin => 0x80000054,
            Coin::Auroracoin => 0x80000055,
            Coin::IXCoin => 0x80000056,
            Coin::Gulden => 0x80000057,
            Coin::BitBean => 0x80000058,
            Coin::Bata => 0x80000059,
            Coin::Myriadcoin => 0x8000005a,
            Coin::BitSend => 0x8000005b,
            Coin::Unobtanium => 0x8000005c,
            Coin::MasterTrader => 0x8000005d,
            Coin::GoldBlocks => 0x8000005e,
            Coin::Saham => 0x8000005f,
            Coin::Chronos => 0x80000060,
            Coin::Ubiquoin => 0x80000061,
            Coin::Evotion => 0x80000062,
            Coin::SaveTheOcean => 0x80000063,
            Coin::BigUp => 0x80000064,
            Coin::GameCredits => 0x80000065,
            Coin::Dollarcoins => 0x80000066,
            Coin::Zayedcoin => 0x80000067,
            Coin::Dubaicoin => 0x80000068,
            Coin::Stratis => 0x80000069,
            Coin::Shilling => 0x8000006a,
            Coin::MarsCoin => 0x8000006b,
            Coin::Ubiq => 0x8000006c,
            Coin::Pesetacoin => 0x8000006d,
            Coin::Neurocoin => 0x8000006e,
            Coin::ARK => 0x8000006f,
            Coin::UltimateSecureCashMain => 0x80000070,
            Coin::Hempcoin => 0x80000071,
            Coin::Linx => 0x80000072,
            Coin::Ecoin => 0x80000073,
            Coin::Denarius => 0x80000074,
            Coin::Pinkcoin => 0x80000075,
            Coin::PiggyCoin => 0x80000076,
            Coin::Pivx => 0x80000077,
            Coin::Flashcoin => 0x80000078,
            Coin::Zencash => 0x80000079,
            Coin::Putincoin => 0x8000007a,
            Coin::BitZeny => 0x8000007b,
            Coin::Unify => 0x8000007c,
            Coin::StealthCoin => 0x8000007d,
            Coin::BreakoutCoin => 0x8000007e,
            Coin::Vcash => 0x8000007f,
            Coin::Monero => 0x80000080,
            Coin::Voxels => 0x80000081,
            Coin::NavCoin => 0x80000082,
            Coin::FactomFactoids => 0x80000083,
            Coin::FactomEntryCredits => 0x80000084,
            Coin::Zcash => 0x80000085,
            Coin::Lisk => 0x80000086,
            Coin::Steem => 0x80000087,
            Coin::ZCoin => 0x80000088,
            Coin::Rootstock => 0x80000089,
            Coin::RealPointCoin => 0x8000008b,
            Coin::LBRYCredits => 0x8000008c,
            Coin::Komodo => 0x8000008d,
            Coin::BisqToken => 0x8000008e,
            Coin::Riecoin => 0x8000008f,
            Coin::Ripple => 0x80000090,
            Coin::BitcoinCash => 0x80000091,
            Coin::Neblio => 0x80000092,
            Coin::ZClassic => 0x80000093,
            Coin::StellarLumens => 0x80000094,
            Coin::WhaleCoin => 0x80000096,
            Coin::EuropeCoin => 0x80000097,
            Coin::Diamond => 0x80000098,
            Coin::Bytom => 0x80000099,
            Coin::Biocoin => 0x8000009a,
            Coin::Whitecoin => 0x8000009b,
            Coin::BitcoinGold => 0x8000009c,
            Coin::SuperSkynet => 0x8000009e,
            Coin::TOACoin => 0x8000009f,
            Coin::Bitcore => 0x800000a0,
            Coin::Adcoin => 0x800000a1,
            Coin::Bridgecoin => 0x800000a2,
            Coin::Ellaism => 0x800000a3,
            Coin::Pirl => 0x800000a4,
            Coin::RaiBlocks => 0x800000a5,
            Coin::Vivo => 0x800000a6,
            Coin::Firstcoin => 0x800000a7,
            Coin::Helleniccoin => 0x800000a8,
            Coin::BUZZ => 0x800000a9,
            Coin::Ember => 0x800000aa,
            Coin::Hcash => 0x800000ab,
            Coin::HTMLCOIN => 0x800000ac,
            Coin::Obsidian => 0x800000ad,
            Coin::OnixCoin => 0x800000ae,
            Coin::Ravencoin => 0x800000af,
            Coin::GoByte => 0x800000b0,
            Coin::BitcoinZ => 0x800000b1,
            Coin::Poa => 0x800000b2,
            Coin::NewYorkCoin => 0x800000b3,
            Coin::MarteXcoin => 0x800000b4,
            Coin::Wincoin => 0x800000b5,
            Coin::Minexcoin => 0x800000b6,
            Coin::BitcoinPrivate => 0x800000b7,
            Coin::Musicoin => 0x800000b8,
            Coin::WorldBitcoin => 0x800000bc,
            Coin::Omni => 0x800000c8,
            Coin::BoxyCoin => 0x800000d7,
            Coin::BitcoinGreen => 0x800000de,
            Coin::AskCoin => 0x800000df,
            Coin::Smartcash => 0x800000e0,
            Coin::XUEZ => 0x800000e1,
            Coin::Varda => 0x800000e9,
            Coin::BitcoinNano => 0x80000100,
            Coin::Blocknet => 0x80000148,
            Coin::MemCoin => 0x8000014d,
            Coin::Phore => 0x800001bc,
            Coin::Koto => 0x800001fe,
            Coin::Radiant => 0x80000200,
            Coin::BitcoinSmart => 0x8000022b,
            Coin::Achain => 0x8000029a,
            Coin::BitcoinWorld => 0x80000309,
            Coin::NEO => 0x80000378,
            Coin::BitcoinDiamond => 0x800003e7,
            Coin::BitcoinNew => 0x800003e8,
            Coin::BigBitcoin => 0x80000457,
            Coin::BitcoinCandy => 0x80000479,
            Coin::Defcoin => 0x80000539,
            Coin::Cardano => 0x80000717,
            Coin::HOdlcoin => 0x800007c5,
            Coin::Axe => 0x80001092,
            Coin::BitcoinPizza => 0x80001a0a,
            Coin::BitcoinQuark => 0x80002093,
            Coin::SuperBitcoin => 0x800022b8,
            Coin::BitcoinPay => 0x80002327,
            Coin::BitcoinFaith => 0x800026a0,
            Coin::Bitvote => 0x8000270f,
            Coin::Wanchain => 0x8057414e,
            Coin::Waves => 0x80579bfc,
            Coin::__Nonexhaustive => unreachable!(),
        }
    }

    /// Gets the coin index for
    /// [registered coin types](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    pub fn index(self) -> usize {
        match self {
            Coin::Bitcoin => 0,
            Coin::Testnet => 1,
            Coin::Litecoin => 2,
            Coin::Dogecoin => 3,
            Coin::Reddcoin => 4,
            Coin::Dash => 5,
            Coin::Peercoin => 6,
            Coin::Namecoin => 7,
            Coin::Feathercoin => 8,
            Coin::Counterparty => 9,
            Coin::Blackcoin => 10,
            Coin::NuShares => 11,
            Coin::NuBits => 12,
            Coin::Mazacoin => 13,
            Coin::Viacoin => 14,
            Coin::ClearingHouse => 15,
            Coin::Rubycoin => 16,
            Coin::Groestlcoin => 17,
            Coin::Digitalcoin => 18,
            Coin::Cannacoin => 19,
            Coin::DigiByte => 20,
            Coin::Monacoin => 22,
            Coin::Clams => 23,
            Coin::Primecoin => 24,
            Coin::Neoscoin => 25,
            Coin::Jumbucks => 26,
            Coin::ZiftCoin => 27,
            Coin::Vertcoin => 28,
            Coin::NXT => 29,
            Coin::Burst => 30,
            Coin::MonetaryUnit => 31,
            Coin::Zoom => 32,
            Coin::Vpncoin => 33,
            Coin::CanadaEcoin => 34,
            Coin::ShadowCash => 35,
            Coin::ParkByte => 36,
            Coin::Pandacoin => 37,
            Coin::StartCoin => 38,
            Coin::MOIN => 39,
            Coin::Expanse => 40,
            Coin::Decred => 42,
            Coin::NEM => 43,
            Coin::Particl => 44,
            Coin::Argentum => 45,
            Coin::Shreeji => 48,
            Coin::GcrCoin => 49,
            Coin::Novacoin => 50,
            Coin::Asiacoin => 51,
            Coin::Bitcoindark => 52,
            Coin::Dopecoin => 53,
            Coin::Templecoin => 54,
            Coin::AIB => 55,
            Coin::EDRCoin => 56,
            Coin::Syscoin => 57,
            Coin::Solarcoin => 58,
            Coin::Smileycoin => 59,
            Coin::Ethereum => 60,
            Coin::EthereumClassic => 61,
            Coin::Pesobit => 62,
            Coin::Landcoin => 63,
            Coin::Bitcoinplus => 65,
            Coin::InternetofPeople => 66,
            Coin::Nexus => 67,
            Coin::InsaneCoin => 68,
            Coin::OkCash => 69,
            Coin::BritCoin => 70,
            Coin::Compcoin => 71,
            Coin::Crown => 72,
            Coin::BelaCoin => 73,
            Coin::VirtualCash => 74,
            Coin::FujiCoin => 75,
            Coin::MIX => 76,
            Coin::Verge => 77,
            Coin::ElectronicGulden => 78,
            Coin::ClubCoin => 79,
            Coin::RichCoin => 80,
            Coin::Potcoin => 81,
            Coin::Quarkcoin => 82,
            Coin::Terracoin => 83,
            Coin::Gridcoin => 84,
            Coin::Auroracoin => 85,
            Coin::IXCoin => 86,
            Coin::Gulden => 87,
            Coin::BitBean => 88,
            Coin::Bata => 89,
            Coin::Myriadcoin => 90,
            Coin::BitSend => 91,
            Coin::Unobtanium => 92,
            Coin::MasterTrader => 93,
            Coin::GoldBlocks => 94,
            Coin::Saham => 95,
            Coin::Chronos => 96,
            Coin::Ubiquoin => 97,
            Coin::Evotion => 98,
            Coin::SaveTheOcean => 99,
            Coin::BigUp => 100,
            Coin::GameCredits => 101,
            Coin::Dollarcoins => 102,
            Coin::Zayedcoin => 103,
            Coin::Dubaicoin => 104,
            Coin::Stratis => 105,
            Coin::Shilling => 106,
            Coin::MarsCoin => 107,
            Coin::Ubiq => 108,
            Coin::Pesetacoin => 109,
            Coin::Neurocoin => 110,
            Coin::ARK => 111,
            Coin::UltimateSecureCashMain => 112,
            Coin::Hempcoin => 113,
            Coin::Linx => 114,
            Coin::Ecoin => 115,
            Coin::Denarius => 116,
            Coin::Pinkcoin => 117,
            Coin::PiggyCoin => 118,
            Coin::Pivx => 119,
            Coin::Flashcoin => 120,
            Coin::Zencash => 121,
            Coin::Putincoin => 122,
            Coin::BitZeny => 123,
            Coin::Unify => 124,
            Coin::StealthCoin => 125,
            Coin::BreakoutCoin => 126,
            Coin::Vcash => 127,
            Coin::Monero => 128,
            Coin::Voxels => 129,
            Coin::NavCoin => 130,
            Coin::FactomFactoids => 131,
            Coin::FactomEntryCredits => 132,
            Coin::Zcash => 133,
            Coin::Lisk => 134,
            Coin::Steem => 135,
            Coin::ZCoin => 136,
            Coin::Rootstock => 137,
            Coin::RealPointCoin => 139,
            Coin::LBRYCredits => 140,
            Coin::Komodo => 141,
            Coin::BisqToken => 142,
            Coin::Riecoin => 143,
            Coin::Ripple => 144,
            Coin::BitcoinCash => 145,
            Coin::Neblio => 146,
            Coin::ZClassic => 147,
            Coin::StellarLumens => 148,
            Coin::WhaleCoin => 150,
            Coin::EuropeCoin => 151,
            Coin::Diamond => 152,
            Coin::Bytom => 153,
            Coin::Biocoin => 154,
            Coin::Whitecoin => 155,
            Coin::BitcoinGold => 156,
            Coin::SuperSkynet => 158,
            Coin::TOACoin => 159,
            Coin::Bitcore => 160,
            Coin::Adcoin => 161,
            Coin::Bridgecoin => 162,
            Coin::Ellaism => 163,
            Coin::Pirl => 164,
            Coin::RaiBlocks => 165,
            Coin::Vivo => 166,
            Coin::Firstcoin => 167,
            Coin::Helleniccoin => 168,
            Coin::BUZZ => 169,
            Coin::Ember => 170,
            Coin::Hcash => 171,
            Coin::HTMLCOIN => 172,
            Coin::Obsidian => 173,
            Coin::OnixCoin => 174,
            Coin::Ravencoin => 175,
            Coin::GoByte => 176,
            Coin::BitcoinZ => 177,
            Coin::Poa => 178,
            Coin::NewYorkCoin => 179,
            Coin::MarteXcoin => 180,
            Coin::Wincoin => 181,
            Coin::Minexcoin => 182,
            Coin::BitcoinPrivate => 183,
            Coin::Musicoin => 184,
            Coin::WorldBitcoin => 188,
            Coin::Omni => 200,
            Coin::BoxyCoin => 215,
            Coin::BitcoinGreen => 222,
            Coin::AskCoin => 223,
            Coin::Smartcash => 224,
            Coin::XUEZ => 225,
            Coin::Varda => 233,
            Coin::BitcoinNano => 256,
            Coin::Blocknet => 328,
            Coin::MemCoin => 333,
            Coin::Phore => 444,
            Coin::Koto => 510,
            Coin::Radiant => 512,
            Coin::BitcoinSmart => 555,
            Coin::Achain => 666,
            Coin::BitcoinWorld => 777,
            Coin::NEO => 888,
            Coin::BitcoinDiamond => 999,
            Coin::BitcoinNew => 1000,
            Coin::BigBitcoin => 1111,
            Coin::BitcoinCandy => 1145,
            Coin::Defcoin => 1337,
            Coin::Cardano => 1815,
            Coin::HOdlcoin => 1989,
            Coin::Axe => 4242,
            Coin::BitcoinPizza => 6666,
            Coin::BitcoinQuark => 8339,
            Coin::SuperBitcoin => 8888,
            Coin::BitcoinPay => 8999,
            Coin::BitcoinFaith => 9888,
            Coin::Bitvote => 9999,
            Coin::Wanchain => 5718350,
            Coin::Waves => 5741564,
            Coin::__Nonexhaustive => unreachable!(),
        }
    }
}
