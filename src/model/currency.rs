use serde;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Currency {
    UnitedArabEmiratesDirham,
    AfghanistanAfghani,
    AlbaniaLek,
    ArmeniaDram,
    NetherlandsAntillesGuilder,
    AngolaKwanza,
    ArgentinaPeso,
    AustraliaDollar,
    ArubaGuilder,
    AzerbaijanNewManat,
    BosniaAndHerzegovinaConvertibleMarka,
    BarbadosDollar,
    BangladeshTaka,
    BulgariaLev,
    BahrainDinar,
    BurundiFranc,
    BermudaDollar,
    BruneiDarussalamDollar,
    BoliviaBoliviano,
    BrazilReal,
    BahamasDollar,
    BhutanNgultrum,
    BotswanaPula,
    BelarusRuble,
    BelizeDollar,
    CanadaDollar,
    CongoKinshasaFranc,
    SwitzerlandFranc,
    ChilePeso,
    ChinaYuanRenminbi,
    ColombiaPeso,
    CostaRicaColon,
    CubaConvertiblePeso,
    CubaPeso,
    CapeVerdeEscudo,
    CzechRepublicKoruna,
    DjiboutiFranc,
    DenmarkKrone,
    DominicanRepublicPeso,
    AlgeriaDinar,
    EgyptPound,
    EritreaNakfa,
    EthiopiaBirr,
    EuroMemberCountries,
    FijiDollar,
    FalklandIslandsPound,
    UnitedKingdomPound,
    GeorgiaLari,
    GuernseyPound,
    GhanaCedi,
    GibraltarPound,
    GambiaDalasi,
    GuineaFranc,
    GuatemalaQuetzal,
    GuyanaDollar,
    HongKongDollar,
    HondurasLempira,
    CroatiaKuna,
    HaitiGourde,
    HungaryForint,
    IndonesiaRupiah,
    IsraelShekel,
    IsleOfManPound,
    IndiaRupee,
    IraqDinar,
    IranRial,
    IcelandKrona,
    JerseyPound,
    JamaicaDollar,
    JordanDinar,
    JapanYen,
    KenyaShilling,
    KyrgyzstanSom,
    CambodiaRiel,
    ComorosFranc,
    NorthKoreaWon,
    SouthKoreaWon,
    KuwaitDinar,
    CaymanIslandsDollar,
    KazakhstanTenge,
    LaosKip,
    LebanonPound,
    SriLankaRupee,
    LiberiaDollar,
    LesothoLoti,
    LibyaDinar,
    MoroccoDirham,
    MoldovaLeu,
    MadagascarAriary,
    MacedoniaDenar,
    MyanmarKyat,
    MongoliaTughrik,
    MacauPataca,
    MauritaniaOuguiya,
    MauritiusRupee,
    MaldivesRufiyaa,
    MalawiKwacha,
    MexicoPeso,
    MalaysiaRinggit,
    MozambiqueMetical,
    NamibiaDollar,
    NigeriaNaira,
    NicaraguaCordoba,
    NorwayKrone,
    NepalRupee,
    NewZealandDollar,
    OmanRial,
    PanamaBalboa,
    PeruSol,
    PapuaNewGuineaKina,
    PhilippinesPeso,
    PakistanRupee,
    PolandZloty,
    ParaguayGuarani,
    QatarRiyal,
    RomaniaNewLeu,
    SerbiaDinar,
    RussiaRuble,
    RwandaFranc,
    SaudiArabiaRiyal,
    SolomonIslandsDollar,
    SeychellesRupee,
    SudanPound,
    SwedenKrona,
    SingaporeDollar,
    SaintHelenaPound,
    SierraLeoneLeone,
    SomaliaShilling,
    SeborgaLuigino,
    SurinameDollar,
    SaoTomeAndPrincipeDobra,
    ElSalvadorColon,
    SyriaPound,
    SwazilandLilangeni,
    ThailandBaht,
    TajikistanSomoni,
    TurkmenistanManat,
    TunisiaDinar,
    TongaPaanga,
    TurkeyLira,
    TrinidadAndTobagoDollar,
    TuvaluDollar,
    TaiwanNewDollar,
    TanzaniaShilling,
    UkraineHryvnia,
    UgandaShilling,
    UnitedStatesDollar,
    UruguayPeso,
    UzbekistanSom,
    VenezuelaBolivar,
    VietNamDong,
    VanuatuVatu,
    SamoaTala,
    CentralAfricanCfaFranc,
    WestAfricanCfaFranc,
    CfpFranc,
    EastCaribbeanDollar,
    YemenRial,
    SouthAfricaRand,
    ZambiaKwacha,
    ZimbabweDollar,
    Unknown(String)
}

use self::Currency::*;
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnitedArabEmiratesDirham             => write!(f, "aed"),
            AfghanistanAfghani                   => write!(f, "afn"),
            AlbaniaLek                           => write!(f, "all"),
            ArmeniaDram                          => write!(f, "amd"),
            NetherlandsAntillesGuilder           => write!(f, "ang"),
            AngolaKwanza                         => write!(f, "aoa"),
            ArgentinaPeso                        => write!(f, "ars"),
            AustraliaDollar                      => write!(f, "aud"),
            ArubaGuilder                         => write!(f, "awg"),
            AzerbaijanNewManat                   => write!(f, "azn"),
            BosniaAndHerzegovinaConvertibleMarka => write!(f, "bam"),
            BarbadosDollar                       => write!(f, "bbd"),
            BangladeshTaka                       => write!(f, "bdt"),
            BulgariaLev                          => write!(f, "bgn"),
            BahrainDinar                         => write!(f, "bhd"),
            BurundiFranc                         => write!(f, "bif"),
            BermudaDollar                        => write!(f, "bmd"),
            BruneiDarussalamDollar               => write!(f, "bnd"),
            BoliviaBoliviano                     => write!(f, "bob"),
            BrazilReal                           => write!(f, "brl"),
            BahamasDollar                        => write!(f, "bsd"),
            BhutanNgultrum                       => write!(f, "btn"),
            BotswanaPula                         => write!(f, "bwp"),
            BelarusRuble                         => write!(f, "byr"),
            BelizeDollar                         => write!(f, "bzd"),
            CanadaDollar                         => write!(f, "cad"),
            CongoKinshasaFranc                   => write!(f, "cdf"),
            SwitzerlandFranc                     => write!(f, "chf"),
            ChilePeso                            => write!(f, "clp"),
            ChinaYuanRenminbi                    => write!(f, "cny"),
            ColombiaPeso                         => write!(f, "cop"),
            CostaRicaColon                       => write!(f, "crc"),
            CubaConvertiblePeso                  => write!(f, "cuc"),
            CubaPeso                             => write!(f, "cup"),
            CapeVerdeEscudo                      => write!(f, "cve"),
            CzechRepublicKoruna                  => write!(f, "czk"),
            DjiboutiFranc                        => write!(f, "djf"),
            DenmarkKrone                         => write!(f, "dkk"),
            DominicanRepublicPeso                => write!(f, "dop"),
            AlgeriaDinar                         => write!(f, "dzd"),
            EgyptPound                           => write!(f, "egp"),
            EritreaNakfa                         => write!(f, "ern"),
            EthiopiaBirr                         => write!(f, "etb"),
            EuroMemberCountries                  => write!(f, "eur"),
            FijiDollar                           => write!(f, "fjd"),
            FalklandIslandsPound                 => write!(f, "fkp"),
            UnitedKingdomPound                   => write!(f, "gbp"),
            GeorgiaLari                          => write!(f, "gel"),
            GuernseyPound                        => write!(f, "ggp"),
            GhanaCedi                            => write!(f, "ghs"),
            GibraltarPound                       => write!(f, "gip"),
            GambiaDalasi                         => write!(f, "gmd"),
            GuineaFranc                          => write!(f, "gnf"),
            GuatemalaQuetzal                     => write!(f, "gtq"),
            GuyanaDollar                         => write!(f, "gyd"),
            HongKongDollar                       => write!(f, "hkd"),
            HondurasLempira                      => write!(f, "hnl"),
            CroatiaKuna                          => write!(f, "hrk"),
            HaitiGourde                          => write!(f, "htg"),
            HungaryForint                        => write!(f, "huf"),
            IndonesiaRupiah                      => write!(f, "idr"),
            IsraelShekel                         => write!(f, "ils"),
            IsleOfManPound                       => write!(f, "imp"),
            IndiaRupee                           => write!(f, "inr"),
            IraqDinar                            => write!(f, "iqd"),
            IranRial                             => write!(f, "irr"),
            IcelandKrona                         => write!(f, "isk"),
            JerseyPound                          => write!(f, "jep"),
            JamaicaDollar                        => write!(f, "jmd"),
            JordanDinar                          => write!(f, "jod"),
            JapanYen                             => write!(f, "jpy"),
            KenyaShilling                        => write!(f, "kes"),
            KyrgyzstanSom                        => write!(f, "kgs"),
            CambodiaRiel                         => write!(f, "khr"),
            ComorosFranc                         => write!(f, "kmf"),
            NorthKoreaWon                        => write!(f, "kpw"),
            SouthKoreaWon                        => write!(f, "krw"),
            KuwaitDinar                          => write!(f, "kwd"),
            CaymanIslandsDollar                  => write!(f, "kyd"),
            KazakhstanTenge                      => write!(f, "kzt"),
            LaosKip                              => write!(f, "lak"),
            LebanonPound                         => write!(f, "lbp"),
            SriLankaRupee                        => write!(f, "lkr"),
            LiberiaDollar                        => write!(f, "lrd"),
            LesothoLoti                          => write!(f, "lsl"),
            LibyaDinar                           => write!(f, "lyd"),
            MoroccoDirham                        => write!(f, "mad"),
            MoldovaLeu                           => write!(f, "mdl"),
            MadagascarAriary                     => write!(f, "mga"),
            MacedoniaDenar                       => write!(f, "mkd"),
            MyanmarKyat                          => write!(f, "mmk"),
            MongoliaTughrik                      => write!(f, "mnt"),
            MacauPataca                          => write!(f, "mop"),
            MauritaniaOuguiya                    => write!(f, "mro"),
            MauritiusRupee                       => write!(f, "mur"),
            MaldivesRufiyaa                      => write!(f, "mvr"),
            MalawiKwacha                         => write!(f, "mwk"),
            MexicoPeso                           => write!(f, "mxn"),
            MalaysiaRinggit                      => write!(f, "myr"),
            MozambiqueMetical                    => write!(f, "mzn"),
            NamibiaDollar                        => write!(f, "nad"),
            NigeriaNaira                         => write!(f, "ngn"),
            NicaraguaCordoba                     => write!(f, "nio"),
            NorwayKrone                          => write!(f, "nok"),
            NepalRupee                           => write!(f, "npr"),
            NewZealandDollar                     => write!(f, "nzd"),
            OmanRial                             => write!(f, "omr"),
            PanamaBalboa                         => write!(f, "pab"),
            PeruSol                              => write!(f, "pen"),
            PapuaNewGuineaKina                   => write!(f, "pgk"),
            PhilippinesPeso                      => write!(f, "php"),
            PakistanRupee                        => write!(f, "pkr"),
            PolandZloty                          => write!(f, "pln"),
            ParaguayGuarani                      => write!(f, "pyg"),
            QatarRiyal                           => write!(f, "qar"),
            RomaniaNewLeu                        => write!(f, "ron"),
            SerbiaDinar                          => write!(f, "rsd"),
            RussiaRuble                          => write!(f, "rub"),
            RwandaFranc                          => write!(f, "rwf"),
            SaudiArabiaRiyal                     => write!(f, "sar"),
            SolomonIslandsDollar                 => write!(f, "sbd"),
            SeychellesRupee                      => write!(f, "scr"),
            SudanPound                           => write!(f, "sdg"),
            SwedenKrona                          => write!(f, "sek"),
            SingaporeDollar                      => write!(f, "sgd"),
            SaintHelenaPound                     => write!(f, "shp"),
            SierraLeoneLeone                     => write!(f, "sll"),
            SomaliaShilling                      => write!(f, "sos"),
            SeborgaLuigino                       => write!(f, "spl"),
            SurinameDollar                       => write!(f, "srd"),
            SaoTomeAndPrincipeDobra              => write!(f, "std"),
            ElSalvadorColon                      => write!(f, "svc"),
            SyriaPound                           => write!(f, "syp"),
            SwazilandLilangeni                   => write!(f, "szl"),
            ThailandBaht                         => write!(f, "thb"),
            TajikistanSomoni                     => write!(f, "tjs"),
            TurkmenistanManat                    => write!(f, "tmt"),
            TunisiaDinar                         => write!(f, "tnd"),
            TongaPaanga                          => write!(f, "top"),
            TurkeyLira                           => write!(f, "try"),
            TrinidadAndTobagoDollar              => write!(f, "ttd"),
            TuvaluDollar                         => write!(f, "tvd"),
            TaiwanNewDollar                      => write!(f, "twd"),
            TanzaniaShilling                     => write!(f, "tzs"),
            UkraineHryvnia                       => write!(f, "uah"),
            UgandaShilling                       => write!(f, "ugx"),
            UnitedStatesDollar                   => write!(f, "usd"),
            UruguayPeso                          => write!(f, "uyu"),
            UzbekistanSom                        => write!(f, "uzs"),
            VenezuelaBolivar                     => write!(f, "vef"),
            VietNamDong                          => write!(f, "vnd"),
            VanuatuVatu                          => write!(f, "vuv"),
            SamoaTala                            => write!(f, "wst"),
            CentralAfricanCfaFranc               => write!(f, "xaf"),
            WestAfricanCfaFranc                  => write!(f, "xof"),
            CfpFranc                             => write!(f, "xpf"),
            EastCaribbeanDollar                  => write!(f, "xcd"),
            YemenRial                            => write!(f, "yer"),
            SouthAfricaRand                      => write!(f, "zar"),
            ZambiaKwacha                         => write!(f, "zmw"),
            ZimbabweDollar                       => write!(f, "zwd"),
            Unknown(ref s)                       => write!(f, "{}", s),
        }
    }
}

impl serde::Deserialize for Currency {
    fn deserialize<D>(deserializer: &mut D) -> Result<Currency, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "aed"   => UnitedArabEmiratesDirham,
            "afn"   => AfghanistanAfghani,
            "all"   => AlbaniaLek,
            "amd"   => ArmeniaDram,
            "ang"   => NetherlandsAntillesGuilder,
            "aoa"   => AngolaKwanza,
            "ars"   => ArgentinaPeso,
            "aud"   => AustraliaDollar,
            "awg"   => ArubaGuilder,
            "azn"   => AzerbaijanNewManat,
            "bam"   => BosniaAndHerzegovinaConvertibleMarka,
            "bbd"   => BarbadosDollar,
            "bdt"   => BangladeshTaka,
            "bgn"   => BulgariaLev,
            "bhd"   => BahrainDinar,
            "bif"   => BurundiFranc,
            "bmd"   => BermudaDollar,
            "bnd"   => BruneiDarussalamDollar,
            "bob"   => BoliviaBoliviano,
            "brl"   => BrazilReal,
            "bsd"   => BahamasDollar,
            "btn"   => BhutanNgultrum,
            "bwp"   => BotswanaPula,
            "byr"   => BelarusRuble,
            "bzd"   => BelizeDollar,
            "cad"   => CanadaDollar,
            "cdf"   => CongoKinshasaFranc,
            "chf"   => SwitzerlandFranc,
            "clp"   => ChilePeso,
            "cny"   => ChinaYuanRenminbi,
            "cop"   => ColombiaPeso,
            "crc"   => CostaRicaColon,
            "cuc"   => CubaConvertiblePeso,
            "cup"   => CubaPeso,
            "cve"   => CapeVerdeEscudo,
            "czk"   => CzechRepublicKoruna,
            "djf"   => DjiboutiFranc,
            "dkk"   => DenmarkKrone,
            "dop"   => DominicanRepublicPeso,
            "dzd"   => AlgeriaDinar,
            "egp"   => EgyptPound,
            "ern"   => EritreaNakfa,
            "etb"   => EthiopiaBirr,
            "eur"   => EuroMemberCountries,
            "fjd"   => FijiDollar,
            "fkp"   => FalklandIslandsPound,
            "gbp"   => UnitedKingdomPound,
            "gel"   => GeorgiaLari,
            "ggp"   => GuernseyPound,
            "ghs"   => GhanaCedi,
            "gip"   => GibraltarPound,
            "gmd"   => GambiaDalasi,
            "gnf"   => GuineaFranc,
            "gtq"   => GuatemalaQuetzal,
            "gyd"   => GuyanaDollar,
            "hkd"   => HongKongDollar,
            "hnl"   => HondurasLempira,
            "hrk"   => CroatiaKuna,
            "htg"   => HaitiGourde,
            "huf"   => HungaryForint,
            "idr"   => IndonesiaRupiah,
            "ils"   => IsraelShekel,
            "imp"   => IsleOfManPound,
            "inr"   => IndiaRupee,
            "iqd"   => IraqDinar,
            "irr"   => IranRial,
            "isk"   => IcelandKrona,
            "jep"   => JerseyPound,
            "jmd"   => JamaicaDollar,
            "jod"   => JordanDinar,
            "jpy"   => JapanYen,
            "kes"   => KenyaShilling,
            "kgs"   => KyrgyzstanSom,
            "khr"   => CambodiaRiel,
            "kmf"   => ComorosFranc,
            "kpw"   => NorthKoreaWon,
            "krw"   => SouthKoreaWon,
            "kwd"   => KuwaitDinar,
            "kyd"   => CaymanIslandsDollar,
            "kzt"   => KazakhstanTenge,
            "lak"   => LaosKip,
            "lbp"   => LebanonPound,
            "lkr"   => SriLankaRupee,
            "lrd"   => LiberiaDollar,
            "lsl"   => LesothoLoti,
            "lyd"   => LibyaDinar,
            "mad"   => MoroccoDirham,
            "mdl"   => MoldovaLeu,
            "mga"   => MadagascarAriary,
            "mkd"   => MacedoniaDenar,
            "mmk"   => MyanmarKyat,
            "mnt"   => MongoliaTughrik,
            "mop"   => MacauPataca,
            "mro"   => MauritaniaOuguiya,
            "mur"   => MauritiusRupee,
            "mvr"   => MaldivesRufiyaa,
            "mwk"   => MalawiKwacha,
            "mxn"   => MexicoPeso,
            "myr"   => MalaysiaRinggit,
            "mzn"   => MozambiqueMetical,
            "nad"   => NamibiaDollar,
            "ngn"   => NigeriaNaira,
            "nio"   => NicaraguaCordoba,
            "nok"   => NorwayKrone,
            "npr"   => NepalRupee,
            "nzd"   => NewZealandDollar,
            "omr"   => OmanRial,
            "pab"   => PanamaBalboa,
            "pen"   => PeruSol,
            "pgk"   => PapuaNewGuineaKina,
            "php"   => PhilippinesPeso,
            "pkr"   => PakistanRupee,
            "pln"   => PolandZloty,
            "pyg"   => ParaguayGuarani,
            "qar"   => QatarRiyal,
            "ron"   => RomaniaNewLeu,
            "rsd"   => SerbiaDinar,
            "rub"   => RussiaRuble,
            "rwf"   => RwandaFranc,
            "sar"   => SaudiArabiaRiyal,
            "sbd"   => SolomonIslandsDollar,
            "scr"   => SeychellesRupee,
            "sdg"   => SudanPound,
            "sek"   => SwedenKrona,
            "sgd"   => SingaporeDollar,
            "shp"   => SaintHelenaPound,
            "sll"   => SierraLeoneLeone,
            "sos"   => SomaliaShilling,
            "spl"   => SeborgaLuigino,
            "srd"   => SurinameDollar,
            "std"   => SaoTomeAndPrincipeDobra,
            "svc"   => ElSalvadorColon,
            "syp"   => SyriaPound,
            "szl"   => SwazilandLilangeni,
            "thb"   => ThailandBaht,
            "tjs"   => TajikistanSomoni,
            "tmt"   => TurkmenistanManat,
            "tnd"   => TunisiaDinar,
            "top"   => TongaPaanga,
            "try"   => TurkeyLira,
            "ttd"   => TrinidadAndTobagoDollar,
            "tvd"   => TuvaluDollar,
            "twd"   => TaiwanNewDollar,
            "tzs"   => TanzaniaShilling,
            "uah"   => UkraineHryvnia,
            "ugx"   => UgandaShilling,
            "usd"   => UnitedStatesDollar,
            "uyu"   => UruguayPeso,
            "uzs"   => UzbekistanSom,
            "vef"   => VenezuelaBolivar,
            "vnd"   => VietNamDong,
            "vuv"   => VanuatuVatu,
            "wst"   => SamoaTala,
            "xaf"   => CentralAfricanCfaFranc,
            "xof"   => WestAfricanCfaFranc,
            "xpf"   => CfpFranc,
            "xcd"   => EastCaribbeanDollar,
            "yer"   => YemenRial,
            "zar"   => SouthAfricaRand,
            "zmw"   => ZambiaKwacha,
            "zwd"   => ZimbabweDollar,
            unknown => Unknown(String::from(unknown)),
        })
    }
}