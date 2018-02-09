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
    /// [Bitcoin](https://bitcoin.org/), symbol "BTC"
    #[cfg_attr(feature = "serde", serde(rename = "BTC"))]
    Bitcoin,

    /// Testnet (all coins), symbol "TEST"
    #[cfg_attr(feature = "serde", serde(rename = "TEST"))]
    Testnet,

    /// [Litecoin](https://litecoin.org/), symbol "LTC"
    #[cfg_attr(feature = "serde", serde(rename = "LTC"))]
    Litecoin,

    /// [Dogecoin](https://github.com/dogecoin/dogecoin), symbol "DOGE"
    #[cfg_attr(feature = "serde", serde(rename = "DOGE"))]
    Dogecoin,

    /// Reddcoin, symbol "RDD"
    #[cfg_attr(feature = "serde", serde(rename = "RDD"))]
    Reddcoin,

    /// [Dash](https://github.com/dashpay/dash), symbol "DSH"
    #[cfg_attr(feature = "serde", serde(rename = "DSH"))]
    Dash,

    /// [Peercoin](https://peercoin.net/), symbol "PPC"
    #[cfg_attr(feature = "serde", serde(rename = "PPC"))]
    Peercoin,

    /// [Namecoin](http://namecoin.info/), symbol "NMC"
    #[cfg_attr(feature = "serde", serde(rename = "NMC"))]
    Namecoin,

    /// [Feathercoin](https://www.feathercoin.com/), symbol "FTC"
    #[cfg_attr(feature = "serde", serde(rename = "FTC"))]
    Feathercoin,

    /// [Counterparty](http://counterparty.io/), symbol "XCP"
    #[cfg_attr(feature = "serde", serde(rename = "XCP"))]
    Counterparty,

    /// [Blackcoin](http://blackcoin.co/), symbol "BLK"
    #[cfg_attr(feature = "serde", serde(rename = "BLK"))]
    Blackcoin,

    /// [NuShares](https://nubits.com/nushares/introduction), symbol "NSR"
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

    /// [Canada eCoin](https://github.com/Canada-eCoin/), symbol "CDN"
    #[cfg_attr(feature = "serde", serde(rename = "CDN"))]
    CanadaeCoin,

    /// ShadowCash, symbol "SDC"
    #[cfg_attr(feature = "serde", serde(rename = "SDC"))]
    ShadowCash,

    /// [ParkByte](https://github.com/parkbyte/), symbol "PKB"
    #[cfg_attr(feature = "serde", serde(rename = "PKB"))]
    ParkByte,

    /// Pandacoin, symbol "PND"
    #[cfg_attr(feature = "serde", serde(rename = "PND"))]
    Pandacoin,

    /// StartCOIN, symbol "START"
    #[cfg_attr(feature = "serde", serde(rename = "START"))]
    StartCoin,

    /// [MOIN](https://discovermoin.com), symbol "MOIN"
    #[cfg_attr(feature = "serde", serde(rename = "MOIN"))]
    MOIN,

    /// [Expanse](http://www.expanse.tech/), symbol "EXP"
    #[cfg_attr(feature = "serde", serde(rename = "EXP"))]
    Expanse,

    /// [Decred](https://decred.org/), symbol "DCR"
    #[cfg_attr(feature = "serde", serde(rename = "DCR"))]
    Decred,

    /// [NEM](https://github.com/NemProject), symbol "XEM"
    #[cfg_attr(feature = "serde", serde(rename = "XEM"))]
    NEM,

    /// [Particl](https://particl.io/), symbol "PART"
    #[cfg_attr(feature = "serde", serde(rename = "PART"))]
    Particl,

    /// [Argentum](http://www.argentum.io), symbol "ARG"
    #[cfg_attr(feature = "serde", serde(rename = "ARG"))]
    Argentum,

    /// [Shreeji](https://github.com/SMJBIT/SHREEJI), symbol "SHR"
    #[cfg_attr(feature = "serde", serde(rename = "SHR"))]
    Shreeji,

    /// Global Currency Reserve (GCRcoin), symbol "GCR"
    #[cfg_attr(feature = "serde", serde(rename = "GCR"))]
    GcrCoin,

    /// [Novacoin](https://github.com/novacoin-project/novacoin), symbol "NVC"
    #[cfg_attr(feature = "serde", serde(rename = "NVC"))]
    Novacoin,

    /// [Asiacoin](https://github.com/AsiaCoin/AsiaCoinFix), symbol "AC"
    #[cfg_attr(feature = "serde", serde(rename = "AC"))]
    Asiacoin,

    /// [Bitcoindark](https://github.com/jl777/btcd), symbol "BTCD"
    #[cfg_attr(feature = "serde", serde(rename = "BTCD"))]
    Bitcoindark,

    /// [Dopecoin](https://github.com/dopecoin-dev/DopeCoinV3), symbol "DOPE"
    #[cfg_attr(feature = "serde", serde(rename = "DOPE"))]
    Dopecoin,

    /// [Templecoin](https://github.com/9cat/templecoin), symbol "TPC"
    #[cfg_attr(feature = "serde", serde(rename = "TPC"))]
    Templecoin,

    /// [AIB](https://github.com/iobond/aib), symbol "AIB"
    #[cfg_attr(feature = "serde", serde(rename = "AIB"))]
    AIB,

    /// [EDRCoin](https://github.com/EDRCoin/EDRcoin-src), symbol "EDRC"
    #[cfg_attr(feature = "serde", serde(rename = "EDRC"))]
    EDRCoin,

    /// [Syscoin](https://github.com/syscoin/syscoin2), symbol "SYS"
    #[cfg_attr(feature = "serde", serde(rename = "SYS"))]
    Syscoin,

    /// [Solarcoin](https://github.com/onsightit/solarcoin), symbol "SLR"
    #[cfg_attr(feature = "serde", serde(rename = "SLR"))]
    Solarcoin,

    /// [Smileycoin](https://github.com/tutor-web/smileyCoin), symbol "SMLY"
    #[cfg_attr(feature = "serde", serde(rename = "SMLY"))]
    Smileycoin,

    /// [Ethereum](https://ethereum.org/ether), symbol "ETH"
    #[cfg_attr(feature = "serde", serde(rename = "ETH"))]
    Ethereum,

    /// [Ethereum Classic](https://ethereumclassic.github.io), symbol "ETC"
    #[cfg_attr(feature = "serde", serde(rename = "ETC"))]
    EthereumClassic,

    /// [Pesobit](https://github.com/pesobitph/pesobit-source), symbol "PSB"
    #[cfg_attr(feature = "serde", serde(rename = "PSB"))]
    Pesobit,

    /// [Landcoin](http://landcoin.co/), symbol "LDCN"
    #[cfg_attr(feature = "serde", serde(rename = "LDCN"))]
    Landcoin,

    /// [Bitcoinplus](https://bitcoinplus.org), symbol "XBC"
    #[cfg_attr(feature = "serde", serde(rename = "XBC"))]
    Bitcoinplus,

    /// [Internet of People](http://www.fermat.org), symbol "IOP"
    #[cfg_attr(feature = "serde", serde(rename = "IOP"))]
    InternetofPeople,

    /// [Nexus](http://www.nexusearth.com/), symbol "NXS"
    #[cfg_attr(feature = "serde", serde(rename = "NXS"))]
    Nexus,

    /// [InsaneCoin](http://insanecoin.com), symbol "INSN"
    #[cfg_attr(feature = "serde", serde(rename = "INSN"))]
    InsaneCoin,

    /// [OkCash](https://github.com/okcashpro/), symbol "OK"
    #[cfg_attr(feature = "serde", serde(rename = "OK"))]
    OkCash,

    /// [BritCoin](https://britcoin.com), symbol "BRIT"
    #[cfg_attr(feature = "serde", serde(rename = "BRIT"))]
    BritCoin,

    /// [Compcoin](https://compcoin.com), symbol "CMP"
    #[cfg_attr(feature = "serde", serde(rename = "CMP"))]
    Compcoin,

    /// [Crown](http://crown.tech/), symbol "CRW"
    #[cfg_attr(feature = "serde", serde(rename = "CRW"))]
    Crown,

    /// [BelaCoin](http://belacoin.org), symbol "BELA"
    #[cfg_attr(feature = "serde", serde(rename = "BELA"))]
    BelaCoin,

    /// [Virtual Cash](http://www.bitnet.cc/), symbol "VASH"
    #[cfg_attr(feature = "serde", serde(rename = "VASH"))]
    VirtualCash,

    /// [FujiCoin](http://www.fujicoin.org/), symbol "FJC"
    #[cfg_attr(feature = "serde", serde(rename = "FJC"))]
    FujiCoin,

    /// [MIX](https://www.mix-blockchain.org/), symbol "MIX"
    #[cfg_attr(feature = "serde", serde(rename = "MIX"))]
    MIX,

    /// [Verge](https://github.com/vergecurrency/verge/), symbol "XVG"
    #[cfg_attr(feature = "serde", serde(rename = "XVG"))]
    Verge,

    /// [Electronic Gulden](https://egulden.org/), symbol "EFL"
    #[cfg_attr(feature = "serde", serde(rename = "EFL"))]
    ElectronicGulden,

    /// [ClubCoin](https://clubcoin.co/), symbol "CLUB"
    #[cfg_attr(feature = "serde", serde(rename = "CLUB"))]
    ClubCoin,

    /// [RichCoin](https://richcoin.us/), symbol "RICHX"
    #[cfg_attr(feature = "serde", serde(rename = "RICHX"))]
    RichCoin,

    /// [Potcoin](http://potcoin.com/), symbol "POT"
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

    /// [Auroracoin](http://auroracoin.is/), symbol "AUR"
    #[cfg_attr(feature = "serde", serde(rename = "AUR"))]
    Auroracoin,

    /// IXCoin, symbol "IXC"
    #[cfg_attr(feature = "serde", serde(rename = "IXC"))]
    IXCoin,

    /// [Gulden](https://Gulden.com/), symbol "NLG"
    #[cfg_attr(feature = "serde", serde(rename = "NLG"))]
    Gulden,

    /// [BitBean](http://bitbean.org/), symbol "BITB"
    #[cfg_attr(feature = "serde", serde(rename = "BITB"))]
    BitBean,

    /// [Bata](http://bata.io/), symbol "BTA"
    #[cfg_attr(feature = "serde", serde(rename = "BTA"))]
    Bata,

    /// [Myriadcoin](http://myriadcoin.org), symbol "XMY"
    #[cfg_attr(feature = "serde", serde(rename = "XMY"))]
    Myriadcoin,

    /// [BitSend](http://bitsend.info), symbol "BSD"
    #[cfg_attr(feature = "serde", serde(rename = "BSD"))]
    BitSend,

    /// [Unobtanium](http://http://unobtanium.uno/), symbol "UNO"
    #[cfg_attr(feature = "serde", serde(rename = "UNO"))]
    Unobtanium,

    /// [MasterTrader](https://github.com/CrypticApplications/MT), symbol "MTR"
    #[cfg_attr(feature = "serde", serde(rename = "MTR"))]
    MasterTrader,

    /// [GoldBlocks](https://github.com/goldblockscoin/goldblocks), symbol "GB"
    #[cfg_attr(feature = "serde", serde(rename = "GB"))]
    GoldBlocks,

    /// [Saham](https://github.com/SahamDev/SahamDev), symbol "SHM"
    #[cfg_attr(feature = "serde", serde(rename = "SHM"))]
    Saham,

    /// [Chronos](https://github.com/chronoscoin/Chronoscoin), symbol "CRX"
    #[cfg_attr(feature = "serde", serde(rename = "CRX"))]
    Chronos,

    /// [Ubiquoin](https://github.com/ubiquoin/ubiq), symbol "BIQ"
    #[cfg_attr(feature = "serde", serde(rename = "BIQ"))]
    Ubiquoin,

    /// [Evotion](https://github.com/evoshiun/Evotion), symbol "EVO"
    #[cfg_attr(feature = "serde", serde(rename = "EVO"))]
    Evotion,

    /// [SaveTheOcean](https://github.com/SaveTheOceanMovement/SaveTheOceanCoin), symbol "STO"
    #[cfg_attr(feature = "serde", serde(rename = "STO"))]
    SaveTheOcean,

    /// [BigUp](https://github.com/BigUps/), symbol "BIGUP"
    #[cfg_attr(feature = "serde", serde(rename = "BIGUP"))]
    BigUp,

    /// [GameCredits](https://github.com/gamecredits-project), symbol "GMC"
    #[cfg_attr(feature = "serde", serde(rename = "GMC"))]
    GameCredits,

    /// [Dollarcoins](https://github.com/dollarcoins/source), symbol "DLC"
    #[cfg_attr(feature = "serde", serde(rename = "DLC"))]
    Dollarcoins,

    /// [Zayedcoin](https://github.com/ZayedCoin/Zayedcoin), symbol "ZYD"
    #[cfg_attr(feature = "serde", serde(rename = "ZYD"))]
    Zayedcoin,

    /// [Dubaicoin](https://github.com/DubaiCoinDev/DubaiCoin), symbol "DBIC"
    #[cfg_attr(feature = "serde", serde(rename = "DBIC"))]
    Dubaicoin,

    /// [Stratis](http://www.stratisplatform.com), symbol "STRAT"
    #[cfg_attr(feature = "serde", serde(rename = "STRAT"))]
    Stratis,

    /// [Shilling](https://github.com/yavwa/Shilling), symbol "SH"
    #[cfg_attr(feature = "serde", serde(rename = "SH"))]
    Shilling,

    /// [MarsCoin](http://www.marscoin.org/), symbol "MARS"
    #[cfg_attr(feature = "serde", serde(rename = "MARS"))]
    MarsCoin,

    /// [Ubiq](https://github.com/Ubiq), symbol "UBQ"
    #[cfg_attr(feature = "serde", serde(rename = "UBQ"))]
    Ubiq,

    /// [Pesetacoin](http://pesetacoin.info/), symbol "PTC"
    #[cfg_attr(feature = "serde", serde(rename = "PTC"))]
    Pesetacoin,

    /// [Neurocoin](https://neurocoin.org), symbol "NRC"
    #[cfg_attr(feature = "serde", serde(rename = "NRC"))]
    Neurocoin,

    /// [ARK](https://ark.io), symbol "ARK"
    #[cfg_attr(feature = "serde", serde(rename = "ARK"))]
    ARK,

    /// [UltimateSecureCashMain](http://ultimatesecurecash.info), symbol "USC"
    #[cfg_attr(feature = "serde", serde(rename = "USC"))]
    UltimateSecureCashMain,

    /// [Hempcoin](http://hempcoin.org), symbol "HMP"
    #[cfg_attr(feature = "serde", serde(rename = "HMP"))]
    Hempcoin,

    /// [Linx](https://mylinx.io), symbol "LINX"
    #[cfg_attr(feature = "serde", serde(rename = "LINX"))]
    Linx,

    /// [Ecoin](https://www.ecoinsource.com), symbol "ECN"
    #[cfg_attr(feature = "serde", serde(rename = "ECN"))]
    Ecoin,

    /// [Denarius](https://denarius.io), symbol "DNR"
    #[cfg_attr(feature = "serde", serde(rename = "DNR"))]
    Denarius,

    /// [Pinkcoin](http://getstarted.with.pink), symbol "PINK"
    #[cfg_attr(feature = "serde", serde(rename = "PINK"))]
    Pinkcoin,

    /// [PiggyCoin](https://www.piggy-coin.com/), symbol "PIGGY"
    #[cfg_attr(feature = "serde", serde(rename = "PIGGY"))]
    PiggyCoin,

    /// [Pivx](https://github.com/PIVX-Project/PIVX), symbol "PIVX"
    #[cfg_attr(feature = "serde", serde(rename = "PIVX"))]
    Pivx,

    /// [Flashcoin](https://flashcoin.io), symbol "FLASH"
    #[cfg_attr(feature = "serde", serde(rename = "FLASH"))]
    Flashcoin,

    /// [Zencash](https://zensystem.io), symbol "ZEN"
    #[cfg_attr(feature = "serde", serde(rename = "ZEN"))]
    Zencash,

    /// [Putincoin](https://putincoin.info), symbol "PUT"
    #[cfg_attr(feature = "serde", serde(rename = "PUT"))]
    Putincoin,

    /// [BitZeny](http://bitzeny.org/), symbol "ZNY"
    #[cfg_attr(feature = "serde", serde(rename = "ZNY"))]
    BitZeny,

    /// [Unify](http://unifycryptocurrency.com), symbol "UNIFY"
    #[cfg_attr(feature = "serde", serde(rename = "UNIFY"))]
    Unify,

    /// [StealthCoin](http://www.stealthcoin.com), symbol "XST"
    #[cfg_attr(feature = "serde", serde(rename = "XST"))]
    StealthCoin,

    /// [Breakout Coin](http://www.breakoutcoin.com), symbol "BRK"
    #[cfg_attr(feature = "serde", serde(rename = "BRK"))]
    BreakoutCoin,

    /// [Vcash](https://vcash.info), symbol "VC"
    #[cfg_attr(feature = "serde", serde(rename = "VC"))]
    Vcash,

    /// [Monero](https://getmonero.org/), symbol "XMR"
    #[cfg_attr(feature = "serde", serde(rename = "XMR"))]
    Monero,

    /// [Aeon](http://www.aeon.cash/), symbol "AEON"
    #[cfg_attr(feature = "serde", serde(rename = "AEON"))]
    Aeon,

    /// [Voxels](https://www.voxelus.com), symbol "VOX"
    #[cfg_attr(feature = "serde", serde(rename = "VOX"))]
    Voxels,

    /// [NavCoin](https://github.com/navcoindev/navcoin2), symbol "NAV"
    #[cfg_attr(feature = "serde", serde(rename = "NAV"))]
    NavCoin,

    /// [Factom Factoids](https://github.com/FactomProject/FactomDocs/blob/master/wallet_info/wallet_test_vectors.md), symbol "FCT"
    #[cfg_attr(feature = "serde", serde(rename = "FCT"))]
    FactomFactoids,

    /// [Factom Entry Credits](https://github.com/FactomProject), symbol "EC"
    #[cfg_attr(feature = "serde", serde(rename = "EC"))]
    FactomEntryCredits,

    /// [Zcash](https://z.cash), symbol "ZEC"
    #[cfg_attr(feature = "serde", serde(rename = "ZEC"))]
    Zcash,

    /// [Lisk](https://lisk.io/), symbol "LSK"
    #[cfg_attr(feature = "serde", serde(rename = "LSK"))]
    Lisk,

    /// [Steem](http://steem.io), symbol "STEEM"
    #[cfg_attr(feature = "serde", serde(rename = "STEEM"))]
    Steem,

    /// [ZCoin](https://zcoin.io), symbol "XZC"
    #[cfg_attr(feature = "serde", serde(rename = "XZC"))]
    ZCoin,

    /// [Rootstock](http://www.rsk.co/), symbol "RSK"
    #[cfg_attr(feature = "serde", serde(rename = "RSK"))]
    Rootstock,

    /// [RealPointCoin](https://github.com/MaxSmile/RealPointCoinQt), symbol "RPT"
    #[cfg_attr(feature = "serde", serde(rename = "RPT"))]
    RealPointCoin,

    /// [LBRY Credits](https://lbry.io/), symbol "LBC"
    #[cfg_attr(feature = "serde", serde(rename = "LBC"))]
    LBRYCredits,

    /// [Komodo](https://komodoplatform.com/), symbol "KMD"
    #[cfg_attr(feature = "serde", serde(rename = "KMD"))]
    Komodo,

    /// [bisq Token](http://bisq.io/), symbol "BSQ"
    #[cfg_attr(feature = "serde", serde(rename = "BSQ"))]
    BisqToken,

    /// [Riecoin](https://github.com/riecoin/riecoin), symbol "RIC"
    #[cfg_attr(feature = "serde", serde(rename = "RIC"))]
    Riecoin,

    /// [Ripple](https://ripple.com), symbol "XRP"
    #[cfg_attr(feature = "serde", serde(rename = "XRP"))]
    Ripple,

    /// [Bitcoin Cash](https://www.bitcoincash.org), symbol "BCH"
    #[cfg_attr(feature = "serde", serde(rename = "BCH"))]
    BitcoinCash,

    /// [Neblio](https://nebl.io), symbol "NEBL"
    #[cfg_attr(feature = "serde", serde(rename = "NEBL"))]
    Neblio,

    /// [ZClassic](http://zclassic.org/), symbol "ZCL"
    #[cfg_attr(feature = "serde", serde(rename = "ZCL"))]
    ZClassic,

    /// [Stellar Lumens](https://www.stellar.org/), symbol "XLM"
    #[cfg_attr(feature = "serde", serde(rename = "XLM"))]
    StellarLumens,

    /// [WhaleCoin](https://whalecoin.org/), symbol "WHL"
    #[cfg_attr(feature = "serde", serde(rename = "WHL"))]
    WhaleCoin,

    /// [EuropeCoin](https://www.europecoin.eu.org/), symbol "ERC"
    #[cfg_attr(feature = "serde", serde(rename = "ERC"))]
    EuropeCoin,

    /// [Diamond](http://bit.diamonds), symbol "DMD"
    #[cfg_attr(feature = "serde", serde(rename = "DMD"))]
    Diamond,

    /// [Bytom](https://bytom.io), symbol "BTM"
    #[cfg_attr(feature = "serde", serde(rename = "BTM"))]
    Bytom,

    /// [Biocoin](https://biocoin.bio), symbol "BIO"
    #[cfg_attr(feature = "serde", serde(rename = "BIO"))]
    Biocoin,

    /// [Whitecoin](https://www.whitecoin.info), symbol "XWC"
    #[cfg_attr(feature = "serde", serde(rename = "XWC"))]
    Whitecoin,

    /// [Bitcoin Gold](http://www.btcgpu.org), symbol "BTG"
    #[cfg_attr(feature = "serde", serde(rename = "BTG"))]
    BitcoinGold,

    /// [SuperSkynet](http://wwww.superskynet.org/), symbol "SSN"
    #[cfg_attr(feature = "serde", serde(rename = "SSN"))]
    SuperSkynet,

    /// [TOACoin](http://www.toacoin.com), symbol "TOA"
    #[cfg_attr(feature = "serde", serde(rename = "TOA"))]
    TOACoin,

    /// [Bitcore](https://bitcore.cc), symbol "BTX"
    #[cfg_attr(feature = "serde", serde(rename = "BTX"))]
    Bitcore,

    /// [Adcoin](https://www.getadcoin.com/), symbol "ACC"
    #[cfg_attr(feature = "serde", serde(rename = "ACC"))]
    Adcoin,

    /// [Bridgecoin](https://bridgecoin.org/), symbol "BCO"
    #[cfg_attr(feature = "serde", serde(rename = "BCO"))]
    Bridgecoin,

    /// [Ellaism](https://ellaism.org), symbol "ELLA"
    #[cfg_attr(feature = "serde", serde(rename = "ELLA"))]
    Ellaism,

    /// [Pirl](https://pirl.io), symbol "PIRL"
    #[cfg_attr(feature = "serde", serde(rename = "PIRL"))]
    Pirl,

    /// [RaiBlocks](https://raiblocks.com), symbol "XRB"
    #[cfg_attr(feature = "serde", serde(rename = "XRB"))]
    RaiBlocks,

    /// [Vivo](https://www.vivocrypto.com/), symbol "VIVO"
    #[cfg_attr(feature = "serde", serde(rename = "VIVO"))]
    Vivo,

    /// [Firstcoin](http://firstcoinproject.com), symbol "FRST"
    #[cfg_attr(feature = "serde", serde(rename = "FRST"))]
    Firstcoin,

    /// [Helleniccoin](http://www.helleniccoin.gr/), symbol "HNC"
    #[cfg_attr(feature = "serde", serde(rename = "HNC"))]
    Helleniccoin,

    /// [BUZZ](http://www.buzzcoin.info/), symbol "BUZZ"
    #[cfg_attr(feature = "serde", serde(rename = "BUZZ"))]
    BUZZ,

    /// [Ember](https://www.embercoin.io/), symbol "MBRS"
    #[cfg_attr(feature = "serde", serde(rename = "MBRS"))]
    Ember,

    /// [Hcash](https://h.cash), symbol "HSR"
    #[cfg_attr(feature = "serde", serde(rename = "HSR"))]
    Hcash,

    /// [HTMLCOIN](https://htmlcoin.com/), symbol "HTML"
    #[cfg_attr(feature = "serde", serde(rename = "HTML"))]
    HTMLCOIN,

    /// [Obsidian](https://obsidianplatform.com/), symbol "ODN"
    #[cfg_attr(feature = "serde", serde(rename = "ODN"))]
    Obsidian,

    /// [OnixCoin](https://www.onixcoin.com/), symbol "ONX"
    #[cfg_attr(feature = "serde", serde(rename = "ONX"))]
    OnixCoin,

    /// [Ravencoin](https://ravencoin.org/), symbol "RVN"
    #[cfg_attr(feature = "serde", serde(rename = "RVN"))]
    Ravencoin,

    /// [GoByte](https://gobyte.network), symbol "GBX"
    #[cfg_attr(feature = "serde", serde(rename = "GBX"))]
    GoByte,

    /// [BitcoinZ](https://btcz.rocks/en/), symbol "BTCZ"
    #[cfg_attr(feature = "serde", serde(rename = "BTCZ"))]
    BitcoinZ,

    /// [Poa](https://poa.network), symbol "POA"
    #[cfg_attr(feature = "serde", serde(rename = "POA"))]
    Poa,

    /// [NewYorkCoin](http://nycoin.net), symbol "NYC"
    #[cfg_attr(feature = "serde", serde(rename = "NYC"))]
    NewYorkCoin,

    /// [MarteXcoin](http://martexcoin.org), symbol "MXT"
    #[cfg_attr(feature = "serde", serde(rename = "MXT"))]
    MarteXcoin,

    /// [Wincoin](https://wincoin.co), symbol "WC"
    #[cfg_attr(feature = "serde", serde(rename = "WC"))]
    Wincoin,

    /// [Minexcoin](https://minexcoin.com), symbol "MNX"
    #[cfg_attr(feature = "serde", serde(rename = "MNX"))]
    Minexcoin,

    /// [Bitcoin Private](https://btcprivate.org), symbol "BTCP"
    #[cfg_attr(feature = "serde", serde(rename = "BTCP"))]
    BitcoinPrivate,

    /// [Musicoin](https://www.musicoin.org), symbol "MUSIC"
    #[cfg_attr(feature = "serde", serde(rename = "MUSIC"))]
    Musicoin,

    /// [World Bitcoin](http://www.wbtcteam.org/), symbol "WBTC"
    #[cfg_attr(feature = "serde", serde(rename = "WBTC"))]
    WorldBitcoin,

    /// [Omni](http://www.omnilayer.org), symbol "OMNI"
    #[cfg_attr(feature = "serde", serde(rename = "OMNI"))]
    Omni,

    /// [BoxyCoin](http://www.boxycoin.org/), symbol "BOXY"
    #[cfg_attr(feature = "serde", serde(rename = "BOXY"))]
    BoxyCoin,

    /// [Bitcoin Green](https://savebitcoin.io), symbol "BITG"
    #[cfg_attr(feature = "serde", serde(rename = "BITG"))]
    BitcoinGreen,

    /// [AskCoin](https://askcoin.org), symbol "ASK"
    #[cfg_attr(feature = "serde", serde(rename = "ASK"))]
    AskCoin,

    /// [Smartcash](https://smartcash.cc), symbol "SMART"
    #[cfg_attr(feature = "serde", serde(rename = "SMART"))]
    Smartcash,

    /// [XUEZ](https://xuezcoin.com), symbol "XUEZ"
    #[cfg_attr(feature = "serde", serde(rename = "XUEZ"))]
    XUEZ,

    /// [Varda](https://varda.io), symbol "VAR"
    #[cfg_attr(feature = "serde", serde(rename = "VAR"))]
    Varda,

    /// [Bitcoin Nano](https://www.btcnano.org), symbol "NANO"
    #[cfg_attr(feature = "serde", serde(rename = "NANO"))]
    BitcoinNano,

    /// [Blocknet](https://blocknet.co/), symbol "BLOCK"
    #[cfg_attr(feature = "serde", serde(rename = "BLOCK"))]
    Blocknet,

    /// [MemCoin](https://memcoin.org), symbol "MEM"
    #[cfg_attr(feature = "serde", serde(rename = "MEM"))]
    MemCoin,

    /// [Phore](https://phore.io), symbol "PHR"
    #[cfg_attr(feature = "serde", serde(rename = "PHR"))]
    Phore,

    /// [Koto](https://koto.cash/), symbol "KOTO"
    #[cfg_attr(feature = "serde", serde(rename = "KOTO"))]
    Koto,

    /// [Radiant](https://radiant.cash/), symbol "XRD"
    #[cfg_attr(feature = "serde", serde(rename = "XRD"))]
    Radiant,

    /// [Bitcoin Smart](http://bcs.info), symbol "BCS"
    #[cfg_attr(feature = "serde", serde(rename = "BCS"))]
    BitcoinSmart,

    /// [Achain](https://www.achain.com/), symbol "ACT"
    #[cfg_attr(feature = "serde", serde(rename = "ACT"))]
    Achain,

    /// [Bitcoin World](http://btw.one), symbol "BTW"
    #[cfg_attr(feature = "serde", serde(rename = "BTW"))]
    BitcoinWorld,

    /// [NEO](https://neo.org/), symbol "NEO"
    #[cfg_attr(feature = "serde", serde(rename = "NEO"))]
    NEO,

    /// [Bitcoin Diamond](http://btcd.io/), symbol "BCD"
    #[cfg_attr(feature = "serde", serde(rename = "BCD"))]
    BitcoinDiamond,

    /// [Bitcoin New](http://bitcoinnew.org/), symbol "BTN"
    #[cfg_attr(feature = "serde", serde(rename = "BTN"))]
    BitcoinNew,

    /// [Big Bitcoin](http://bigbitcoins.org/), symbol "BBC"
    #[cfg_attr(feature = "serde", serde(rename = "BBC"))]
    BigBitcoin,

    /// [Bitcoin Candy](http://www.bitcoincandy.one), symbol "CDY"
    #[cfg_attr(feature = "serde", serde(rename = "CDY"))]
    BitcoinCandy,

    /// [Defcoin](http://defcoin-ng.org), symbol "DFC"
    #[cfg_attr(feature = "serde", serde(rename = "DFC"))]
    Defcoin,

    /// [Cardano](https://www.cardanohub.org/en/home/), symbol "ADA"
    #[cfg_attr(feature = "serde", serde(rename = "ADA"))]
    Cardano,

    /// [HOdlcoin](https://hodlcoin.com/), symbol "HODL"
    #[cfg_attr(feature = "serde", serde(rename = "HODL"))]
    HOdlcoin,

    /// [Axe](https://github.com/AXErunners/axe), symbol "AXE"
    #[cfg_attr(feature = "serde", serde(rename = "AXE"))]
    Axe,

    /// [Bitcoin Pizza](http://p.top/), symbol "BPA"
    #[cfg_attr(feature = "serde", serde(rename = "BPA"))]
    BitcoinPizza,

    /// [BitcoinQuark](https://www.bitcoinquark.org), symbol "BTQ"
    #[cfg_attr(feature = "serde", serde(rename = "BTQ"))]
    BitcoinQuark,

    /// [Super Bitcoin](https://www.superbtc.org), symbol "SBTC"
    #[cfg_attr(feature = "serde", serde(rename = "SBTC"))]
    SuperBitcoin,

    /// [Bitcoin Pay](http://www.btceasypay.com), symbol "BTP"
    #[cfg_attr(feature = "serde", serde(rename = "BTP"))]
    BitcoinPay,

    /// [Bitcoin Faith](http://bitcoinfaith.org), symbol "BTF"
    #[cfg_attr(feature = "serde", serde(rename = "BTF"))]
    BitcoinFaith,

    /// [Bitvote](www.bitvote.one), symbol "BTV"
    #[cfg_attr(feature = "serde", serde(rename = "BTV"))]
    Bitvote,

    /// [Wanchain](https://wanchain.org/), symbol "WAN"
    #[cfg_attr(feature = "serde", serde(rename = "WAN"))]
    Wanchain,

    /// [Waves](https://wavesplatform.com/), symbol "WAVES"
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
            "aeon" | "AEMON" => Some(Aeon),
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
            Coin::Aeon => "AEON",
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
}

/// List of all [`Coin`]s in the enumeration, in altcoin index order.
///
/// [`Coin`]: ./enum.Coin.html
pub const COINS: [Coin; 212] = [
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
    Coin::Aeon,
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
