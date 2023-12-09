use lazy_static::lazy_static;

lazy_static!{
    pub static ref MD5_REGEX: regex::Regex = regex::Regex::new(r"\b[A-Fa-f0-9]{32}\b").unwrap();
    pub static ref SHA1_REGEX: regex::Regex = regex::Regex::new(r"\b[A-Fa-f0-9]{40}\b").unwrap();
    pub static ref SHA256_REGEX: regex::Regex = regex::Regex::new(r"\b[A-Fa-f0-9]{64}\b").unwrap();
    pub static ref SHA512_REGEX: regex::Regex = regex::Regex::new(r"\b[A-Fa-f0-9]{128}\b").unwrap();

    pub static ref EMAIL_REGEX: regex::Regex = regex::Regex::new(r"\b[\w.]+@[\w.-]+(?:\[?\.\]?)(?:[\w.-]+(?:\[?\.\]?)*)*\b").unwrap();
    pub static ref IPV4_REGEX: regex::Regex = regex::Regex::new(r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\[?\.\]?){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b").unwrap();
    pub static ref IPV6_REGEX: regex::Regex = regex::Regex::new("(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))").unwrap();
    pub static ref URL_REGEX: regex::Regex = regex::Regex::new(r"\b\b(?:[fhstu]\S\S?[px]s?)\[?://\]?\w\S+/[\w]+\[?\.\]?(?:[\w\-_ .\[\]])*\b").unwrap();
    pub static ref DOMAIN_REGEX: regex::Regex = regex::Regex::new(r"\b(?:[A-Za-z0-9-]+(\[?\.\]?[A-Za-z0-9-]+)*)\[?\.\]?(?:aaa|aarp|abb|abbott|abbvie|abc|able|abogado|abudhabi|ac|academy|accenture|accountant|accountants|aco|actor|ad|ads|adult|ae|aeg|aero|aetna|af|afl|africa|ag|agakhan|agency|ai|aig|airbus|airforce|airtel|akdn|al|alibaba|alipay|allfinanz|allstate|ally|alsace|alstom|am|amazon|americanexpress|americanfamily|amex|amfam|amica|amsterdam|analytics|android|anquan|anz|ao|aol|apartments|app|apple|aq|aquarelle|ar|arab|aramco|archi|army|arpa|art|arte|as|asda|asia|associates|at|athleta|attorney|au|auction|audi|audible|audio|auspost|author|auto|autos|avianca|aw|aws|ax|axa|az|azure|ba|baby|baidu|banamex|bananarepublic|band|bank|bar|barcelona|barclaycard|barclays|barefoot|bargains|baseball|basketball|bauhaus|bayern|bb|bbc|bbt|bbva|bcg|bcn|bd|be|beats|beauty|beer|bentley|berlin|best|bestbuy|bet|bf|bg|bh|bharti|bi|bible|bid|bike|bing|bingo|bio|biz|bj|black|blackfriday|blockbuster|blog|bloomberg|blue|bm|bms|bmw|bn|bnpparibas|bo|boats|boehringer|bofa|bom|bond|boo|book|booking|bosch|bostik|boston|bot|boutique|box|br|bradesco|bridgestone|broadway|broker|brother|brussels|bs|bt|build|builders|business|buy|buzz|bv|bw|by|bz|bzh|ca|cab|cafe|cal|call|calvinklein|cam|camera|camp|canon|capetown|capital|capitalone|car|caravan|cards|care|career|careers|cars|casa|case|cash|casino|cat|catering|catholic|cba|cbn|cbre|cc|cd|center|ceo|cern|cf|cfa|cfd|cg|ch|chanel|channel|charity|chase|chat|cheap|chintai|christmas|chrome|church|ci|cipriani|circle|cisco|citadel|citi|citic|city|ck|cl|claims|cleaning|click|clinic|clinique|clothing|cloud|club|clubmed|cm|cn|co|coach|codes|coffee|college|cologne|com|comcast|commbank|community|company|compare|computer|comsec|condos|construction|consulting|contact|contractors|cooking|cool|coop|corsica|country|coupon|coupons|courses|cpa|cr|credit|creditcard|creditunion|cricket|crown|crs|cruise|cruises|cu|cuisinella|cv|cw|cx|cy|cymru|cyou|cz|dabur|dad|dance|data|date|dating|datsun|day|dclk|dds|de|deal|dealer|deals|degree|delivery|dell|deloitte|delta|democrat|dental|dentist|desi|design|dev|dhl|diamonds|diet|digital|direct|directory|discount|discover|dish|diy|dj|dk|dm|dnp|do|docs|doctor|dog|domains|dot|download|drive|dtv|dubai|dunlop|dupont|durban|dvag|dvr|dz|earth|eat|ec|eco|edeka|edu|education|ee|eg|email|emerck|energy|engineer|engineering|enterprises|epson|equipment|er|ericsson|erni|es|esq|estate|et|etisalat|eu|eurovision|eus|events|exchange|expert|exposed|express|extraspace|fage|fail|fairwinds|faith|family|fan|fans|farm|farmers|fashion|fast|fedex|feedback|ferrari|ferrero|fi|fidelity|fido|film|final|finance|financial|fire|firestone|firmdale|fish|fishing|fit|fitness|fj|fk|flickr|flights|flir|florist|flowers|fly|fm|fo|foo|food|football|ford|forex|forsale|forum|foundation|fox|fr|free|fresenius|frl|frogans|frontier|ftr|fujitsu|fun|fund|furniture|futbol|fyi|ga|gal|gallery|gallo|gallup|game|games|gap|garden|gay|gb|gbiz|gd|gdn|ge|gea|gent|genting|george|gf|gg|ggee|gh|gi|gift|gifts|gives|giving|gl|glass|gle|global|globo|gm|gmail|gmbh|gmo|gmx|gn|godaddy|gold|goldpoint|golf|goo|goodyear|goog|google|gop|got|gov|gp|gq|gr|grainger|graphics|gratis|green|gripe|grocery|group|gs|gt|gu|guardian|gucci|guge|guide|guitars|guru|gw|gy|hair|hamburg|hangout|haus|hbo|hdfc|hdfcbank|health|healthcare|help|helsinki|here|hermes|hiphop|hisamitsu|hitachi|hiv|hk|hkt|hm|hn|hockey|holdings|holiday|homedepot|homegoods|homes|homesense|honda|horse|hospital|host|hosting|hot|hotels|hotmail|house|how|hr|hsbc|ht|hu|hughes|hyatt|hyundai|ibm|icbc|ice|icu|id|ie|ieee|ifm|ikano|il|im|imamat|imdb|immo|immobilien|in|inc|industries|infiniti|info|ing|ink|institute|insurance|insure|int|international|intuit|investments|io|ipiranga|iq|ir|irish|is|ismaili|ist|istanbul|it|itau|itv|jaguar|java|jcb|je|jeep|jetzt|jewelry|jio|jll|jm|jmp|jnj|jo|jobs|joburg|jot|joy|jp|jpmorgan|jprs|juegos|juniper|kaufen|kddi|ke|kerryhotels|kerrylogistics|kerryproperties|kfh|kg|kh|ki|kia|kids|kim|kindle|kitchen|kiwi|km|kn|koeln|komatsu|kosher|kp|kpmg|kpn|kr|krd|kred|kuokgroup|kw|ky|kyoto|kz|la|lacaixa|lamborghini|lamer|lancaster|land|landrover|lanxess|lasalle|lat|latino|latrobe|law|lawyer|lb|lc|lds|lease|leclerc|lefrak|legal|lego|lexus|lgbt|li|lidl|life|lifeinsurance|lifestyle|lighting|like|lilly|limited|limo|lincoln|link|lipsy|live|living|lk|llc|llp|loan|loans|locker|locus|lol|london|lotte|lotto|love|lpl|lplfinancial|lr|ls|lt|ltd|ltda|lu|lundbeck|luxe|luxury|lv|ly|ma|madrid|maif|maison|makeup|man|management|mango|map|market|marketing|markets|marriott|marshalls|mattel|mba|mc|mckinsey|md|me|med|media|meet|melbourne|meme|memorial|men|menu|merckmsd|mg|mh|miami|microsoft|mil|mini|mint|mit|mitsubishi|mk|ml|mlb|mls|mm|mma|mn|mo|mobi|mobile|moda|moe|moi|mom|monash|money|monster|mormon|mortgage|moscow|moto|motorcycles|mov|movie|mp|mq|mr|ms|msd|mt|mtn|mtr|mu|museum|music|mv|mw|mx|my|mz|na|nab|nagoya|name|natura|navy|nba|nc|ne|nec|net|netbank|netflix|network|neustar|new|news|next|nextdirect|nexus|nf|nfl|ng|ngo|nhk|ni|nico|nike|nikon|ninja|nissan|nissay|nl|no|nokia|norton|now|nowruz|nowtv|np|nr|nra|nrw|ntt|nu|nyc|nz|obi|observer|office|okinawa|olayan|olayangroup|oldnavy|ollo|om|omega|one|ong|onl|online|ooo|open|oracle|orange|org|organic|origins|osaka|otsuka|ott|ovh|pa|page|panasonic|paris|pars|partners|parts|party|pay|pccw|pe|pet|pf|pfizer|pg|ph|pharmacy|phd|philips|phone|photo|photography|photos|physio|pics|pictet|pictures|pid|pin|ping|pink|pioneer|pizza|pk|pl|place|play|playstation|plumbing|plus|pm|pn|pnc|pohl|poker|politie|porn|post|pr|pramerica|praxi|press|prime|pro|prod|productions|prof|progressive|promo|properties|property|protection|pru|prudential|ps|pt|pub|pw|pwc|py|qa|qpon|quebec|quest|racing|radio|re|read|realestate|realtor|realty|recipes|red|redstone|redumbrella|rehab|reise|reisen|reit|reliance|ren|rent|rentals|repair|report|republican|rest|restaurant|review|reviews|rexroth|rich|richardli|ricoh|ril|rio|rip|ro|rocks|rodeo|rogers|room|rs|rsvp|ru|rugby|ruhr|run|rw|rwe|ryukyu|sa|saarland|safe|safety|sakura|sale|salon|samsclub|samsung|sandvik|sandvikcoromant|sanofi|sap|sarl|sas|save|saxo|sb|sbi|sbs|sc|sca|scb|schaeffler|schmidt|scholarships|school|schule|schwarz|science|scot|sd|se|search|seat|secure|security|seek|select|sener|services|seven|sew|sex|sexy|sfr|sg|sh|shangrila|sharp|shaw|shell|shia|shiksha|shoes|shop|shopping|shouji|show|si|silk|sina|singles|site|sj|sk|ski|skin|sky|skype|sl|sling|sm|smart|smile|sn|sncf|so|soccer|social|softbank|software|sohu|solar|solutions|song|sony|soy|spa|space|sport|spot|sr|srl|ss|st|stada|staples|star|statebank|statefarm|stc|stcgroup|stockholm|storage|store|stream|studio|study|style|su|sucks|supplies|supply|support|surf|surgery|suzuki|sv|swatch|swiss|sx|sy|sydney|systems|sz|tab|taipei|talk|taobao|target|tatamotors|tatar|tattoo|tax|taxi|tc|tci|td|tdk|team|tech|technology|tel|temasek|tennis|teva|tf|tg|th|thd|theater|theatre|tiaa|tickets|tienda|tips|tires|tirol|tj|tjmaxx|tjx|tk|tkmaxx|tl|tm|tmall|tn|to|today|tokyo|tools|top|toray|toshiba|total|tours|town|toyota|toys|tr|trade|trading|training|travel|travelers|travelersinsurance|trust|trv|tt|tube|tui|tunes|tushu|tv|tvs|tw|tz|ua|ubank|ubs|ug|uk|unicom|university|uno|uol|ups|us|uy|uz|va|vacations|vana|vanguard|vc|ve|vegas|ventures|verisign|versicherung|vet|vg|vi|viajes|video|vig|viking|villas|vin|vip|virgin|visa|vision|viva|vivo|vlaanderen|vn|vodka|volkswagen|volvo|vote|voting|voto|voyage|vu|wales|walmart|walter|wang|wanggou|watch|watches|weather|weatherchannel|webcam|weber|website|wed|wedding|weibo|weir|wf|whoswho|wien|wiki|williamhill|win|windows|wine|winners|wme|wolterskluwer|woodside|work|works|world|wow|ws|wtc|wtf|xbox|xerox|xfinity|xihuan|xin|xn(?:\-*(?:\w+(?:\-*)*)+)|xxx|xyz|yachts|yahoo|yamaxun|yandex|ye|yodobashi|yoga|yokohama|you|youtube|yt|yun|za|zappos|zara|zero|zip|zm|zone|zuerich|zw)\b").unwrap();

    pub static ref DOC_FILE_REGEX: regex::Regex = regex::Regex::new(r"\b(?:(?:[\w\-_ ]?)+)\[?\.\]?(?:docx|docm|doc|csv|pdf|xlsx|xlsm|xll|xls|rtf|txt|pptx|ppt|pages|keynote|numbers)\b").unwrap();
    pub static ref WIN_FILE_REGEX: regex::Regex = regex::Regex::new(r"\b(?:(?:[\w\-_ ]?)+)\[?\.\]?(?:exe|dll|vbs|sys|bat|ps1|ps|psm|cscript|msi|cab)\b").unwrap();
    pub static ref MAC_FILE_REGEX: regex::Regex = regex::Regex::new(r"\b([%A-Za-z\.\-_/ ]+\.(plist|app|pkg))\b").unwrap();
    pub static ref ZIP_FILE_REGEX: regex::Regex = regex::Regex::new(r"\b(?:[\w\-_ .])*\.(?:zip|zipx|7z|rar|tar|gz)\b").unwrap();
    pub static ref WEB_FILE_REGEX: regex::Regex = regex::Regex::new(r"\b(?:[\w\-_ .])*\.(?:html|htm|php|jsp|asp|url)\b").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const MD5_TEST_HASHES: &str = "
        ,nadklflñkasdlpñma,s d.,añsdm 4fc4bb541B2223c6296204a29fbbcd93 aklñsdlanskldnñasklf
        asdasqweasdfa e91d594e1ac7a9739200a80d6267db0 asdasdasd
        asdasdasdas 324fa95245c4a4930df8eda5e81cfd38 asdasdasd
    ";

    const SHA1_TEST_HASHES: &str = "
        lorem ipsum asdjoasod 0a2cce04c7100543fc666ef1641e2fa31a4a1891 lorem ipsum
        lorem ipsum asdjoasod eb6b67874242e02f54c9082cd01b36ab9c9c634d lorem ipsum
        lorem ipsum asdjoasod 670197d13a7270939157005963c46c54ef628ad0 lorem ipsum
    ";

    const SHA256_TEST_HASHES: &str = "
        lorem ipsum c86e12c8c92af5e1de10865d4356a5c5f1a03bc2929bca115613f2a02bb02a57 lpknasda sdaslkm
        lorem ipsum 3bac84151c65cb94f9a756248927f443255ed2e3170c3d622f39e778b0d3df3 lpknasda sdaslkm
        lorem ipsum 5c34a3e0926762a57247e2b56880038526bdcfd016a07bf51bb4691c028674c lpknasda sdaslkm
    ";

    const SHA512_TEST_HASHES: &str = "
        dalsdlas 22acb2a456d378c5023482377328c43768ad7d6a7d415611d70fe9cbb77d387e419044a6fdeff4fb85d964bb7ffdd8200e19d554709ee49ac98ccc8434f3cb4b sdmalsld
        dalsdlas ee97b6b0245ce0a84cee98dd86de1bb9dbf1c6a80555bdfe5e5bb13f495442b6a9e69942bfa376d046e21f4d153b684714bb657d69fd208ffe8c8b8116fe5e sdmalsld
        dalsdlas 5898a383c00371e77dc95a267d9780a958a74e1c82c3ad6f31fccc4e79fd904bd585549de51ea80d1d32deb6bffa907149f807858a851573ae7f79d471bd0a78 sdmalsld
    ";

    const EMAIL_TEST: &str = "
        asdasdasdasd asdas qwe sada qwasd gbellhouse0@google[.]co[.]jp asdas sdaqweasd
        asdasdasdasd asdas qwe sada qwasd geastup1@ustream[.]tv asdas sdaqweasd
        asdasdasdasd asdas qwe sada qwasd ghubach2@rediff.com asdas sdaqweasd
        asdasdasdasd asdas qwe sada qwasd gbm2@rediff[.]com asdas sdaqweasd
    ";

    const IPV4_TEST: &str = "
        asjdajsd ljqwjeop 147.98.105.49 adasdq okqhwoe. ojkpjasp
        asjdajsd ljqwjeop 194[.]182[.]188[.]178 adasdq okqhwoe. ojkpjasp
        asjdajsd ljqwjeop 128.113.63.83 adasdq okqhwoe. ojkpjasp
        asjdajsd ljqwjeop 257.113.63.83 adasdq okqhwoe. ojkpjasp
        asjdajsd ljqwjeop 500.233.300.83 adasdq okqhwoe. ojkpjasp
    ";

    const IPV6_TEST: &str = "
        lskdalsdl jkaspdjpajsd 7b74:4be3:877a:c7f2:2636:fff1:6484:6a9 ajsodaksd ohjaksdklasdok
        lskdalsdl jkaspdjpajsd 2166:7fec:ca76:7b3b:f774:2db8:8775:dd1c ajsodaksd ohjaksdklasdok
        lskdalsdl jkaspdjpajsd b9ed:5591:87ea:2e06:c6c1:3539:9df7:baeb ajsodaksd ohjaksdklasdok
    ";

    const DOMAIN_TEST: &str = "
        ñlasdlpaplsd asdas google.cn lajsda asdklaskld
        ñlasdlpaplsd asdas ca[.]gov lajsda asdklaskld
        ñlasdlpaplsd asdas google.nl lajsda asdklaskld
        asjdajsd ljqwjeop 128.113.63.83 adasdq okqhwoe. ojkpjasp
        ñlasdlpaplsd asdas google[.]xn--tiq49xqy lajsda asdklaskld
        lajsda asdklaskld google[.]xn--vermgensberater-ctb-asdasd-qweqw lajsda asdklaskld
    ";

    const URL_TEST: &str = "
        https://aol[.]com/nisi/nam.xml?vestibulum=platea&ante=dictumst&ipsum=morbi&primis=vestibulum&in=velit&faucibus=id&orci=pretium&luctus=iaculis&et=diam&ultrices=erat&posuere=fermentum&cubilia=justo&curae=nec&donec=condimentum&pharetra=neque&magna=sapien&vestibulum=placerat&aliquet=ante&ultrices=nulla&erat=justo&tortor=aliquam&sollicitudin=quis&mi=turpis&sit=eget&amet=elit&lobortis=sodales&sapien=scelerisque&sapien=mauris&non=sit&mi=amet&integer=eros&ac=suspendisse&neque=accumsan&duis=tortor&bibendum=quis&morbi=turpis
        hxxps://aol.com/nisi/nam.xml?vestibulum=platea&ante=dictumst&ipsum=morbi&primis=vestibulum&in=velit&faucibus=id&orci=pretium&luctus=iaculis&et=diam&ultrices=erat&posuere=fermentum&cubilia=justo&curae=nec&donec=condimentum&pharetra=neque&magna=sapien&vestibulum=placerat&aliquet=ante&ultrices=nulla&erat=justo&tortor=aliquam&sollicitudin=quis&mi=turpis&sit=eget&amet=elit&lobortis=sodales&sapien=scelerisque&sapien=mauris&non=sit&mi=amet&integer=eros&ac=suspendisse&neque=accumsan&duis=tortor&bibendum=quis&morbi=turpis
        https://technorati[.]com/eros/elementum[.]html?augue=mi&vestibulum=in&ante=porttitor&ipsum=pede&primis=justo&in=eu&faucibus=massa&orci=donec&luctus=dapibus&et=duis&ultrices=at&posuere=velit&cubilia=eu&curae=est&donec=congue&pharetra=elementum&magna=in&vestibulum=hac&aliquet=habitasse&ultrices=platea&erat=dictumst&tortor=morbi&sollicitudin=vestibulum&mi=velit&sit=id&amet=pretium&lobortis=iaculis&sapien=diam&sapien=erat&non=fermentum&mi=justo&integer=nec&ac=condimentum&neque=neque&duis=sapien&bibendum=placerat&morbi=ante&non=nulla&quam=justo&nec=aliquam&dui=quis&luctus=turpis&rutrum=eget&nulla=elit&tellus=sodales&in=scelerisque&sagittis=mauris&dui=sit&vel=amet&nisl=eros&duis=suspendisse&ac=accumsan&nibh=tortor&fusce=quis&lacus=turpis&purus=sed&aliquet=ante&at=vivamus&feugiat=tortor&non=duis&pretium=mattis&quis=egestas&lectus=metus&suspendisse=aenean&potenti=fermentum&in=donec&eleifend=ut&quam=mauris&a=eget&odio=massa&in=tempor&hac=convallis&habitasse=nulla&platea=neque&dictumst=libero&maecenas=convallis&ut=eget&massa=eleifend&quis=luctus&augue=ultricies&luctus=eu&tincidunt=nibh
        hxxp://1und1[.]de/primis/in/faucibus/orci/luctus/et/ultrices.jpg
        hxxp://194[.]182[.]188[.]178/primis/in/faucibus/orci/luctus/et/ultrices[.]jpg
        lajsda asdklaskld google[.]xn--vermgensberater-ctb-asdasd-qweqw lajsda asdklaskld
    ";

    const DOC_FILE_TEST: &str = "
        Venenatis.doc
        Ipsum.xls
        Eros Viverra Eget.ppt
        Tincidunt Eget Tempus.ps1
        Ve hi cu l a Condimentum Cu ra bi tu r.pdf
        Ve hi cu l a Condimentum Cu ra bi tu r.xll
    ";

    const WIN_FILE_TEST: &str = "
        VestibulumAc.exe
        Penatibus.dll
        Vestibulum.vbs
        Tincidunt Eget Tempus.ps1
        Ve hi cu l a Condimentum Cu ra bi tu r.pdf
    ";

    const MAC_FILE_TEST: &str = "
        Malesuada.app
        Dictumst.pkg
        LiberoQuis.plist
        Penatibus.dll
    ";

    const ZIP_FILE_TEST: &str = "
        Turpis.rar
        Rutrum.7z
        Dictumst.pkg
        NuncProin.tar
        NuncProin.tar.gz
        LiberoQuis.plist
    ";

    struct MatchTest<'a> {
        name: &'a str,
        regex: &'a regex::Regex,
        haystack: &'a str,
        expected: usize
    }

    #[test]
    fn test_extract_hashes_from_string() {
        let test_cases = vec![
            MatchTest {
                name: "MD5 regex test",
                regex: &MD5_REGEX,
                haystack: MD5_TEST_HASHES,
                expected: 2
            },
            MatchTest {
                name: "SHA1 regex test",
                regex: &SHA1_REGEX,
                haystack: SHA1_TEST_HASHES,
                expected: 3
            },
            MatchTest {
                name: "SHA256 regex test",
                regex: &SHA256_REGEX,
                haystack: SHA256_TEST_HASHES,
                expected: 1
            },
            MatchTest {
                name: "SHA512 regex test",
                regex: &SHA512_REGEX,
                haystack: SHA512_TEST_HASHES,
                expected: 2
            },
            MatchTest {
                name: "Bogus SHA512 regex test",
                regex: &SHA512_REGEX,
                haystack: MD5_TEST_HASHES,
                expected: 0
            },
        ];

        run_test_cases(test_cases);
    }

    #[test]
    fn test_extract_network_iocs_from_string() {
        let test_cases = vec![
            MatchTest {
                name: "Email regex test",
                regex: &EMAIL_REGEX,
                haystack: EMAIL_TEST,
                expected: 4
            },
            MatchTest {
                name: "IPv4 regex test",
                regex: &IPV4_REGEX,
                haystack: IPV4_TEST,
                expected: 3
            },
            MatchTest {
                name: "IPv6 regex test",
                regex: &IPV6_REGEX,
                haystack: IPV6_TEST,
                expected: 3
            },
            MatchTest {
                name: "Domain regex test",
                regex: &DOMAIN_REGEX,
                haystack: DOMAIN_TEST,
                expected: 5
            },
            MatchTest {
                name: "URL regex test",
                regex: &URL_REGEX,
                haystack: URL_TEST,
                expected: 5
            }
        ];

        run_test_cases(test_cases);
    }

    #[test]
    fn test_extract_filenames_from_string() {
        let test_cases = vec![
            MatchTest {
                name: "Doc files regex test",
                regex: &DOC_FILE_REGEX,
                haystack: DOC_FILE_TEST,
                expected: 5
            },
            MatchTest {
                name: "Win files regex test",
                regex: &WIN_FILE_REGEX,
                haystack: WIN_FILE_TEST,
                expected: 4
            },
            MatchTest {
                name: "Mac files regex test",
                regex: &MAC_FILE_REGEX,
                haystack: MAC_FILE_TEST,
                expected: 3
            },
            MatchTest {
                name: "Zip files regex test",
                regex: &ZIP_FILE_REGEX,
                haystack: ZIP_FILE_TEST,
                expected: 4
            },
        ];

        run_test_cases(test_cases);
    }

    fn run_test_cases(test_cases: Vec<MatchTest>) {
        for test in test_cases {
            let test_matches = test.regex.find_iter(test.haystack)
                .map(|r_match| r_match.as_str())
                .collect::<Vec<&str>>()
                .len();

            assert_eq!(
                test_matches,
                test.expected,
                "{0} result differs from expected. Got: {test_matches}. Expected: {1}",
                test.name,
                test.expected
            );
        }
    }
}