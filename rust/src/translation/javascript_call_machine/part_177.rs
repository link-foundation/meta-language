// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result;
use super::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Result;
use super::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Result;
use super::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Result;
use super::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result;
use super::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result;
use super::read_unicode_case_lower_indexes;
use super::read_unicode_case_lower_ranges;
use super::read_unicode_case_lower_values;
use super::read_unicode_case_upper_indexes;
use super::read_unicode_case_upper_ranges;
use super::read_unicode_case_upper_values;
use super::source_ml_source1_temporary_accept_unicode_case_property_loop1351;
use super::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1356;
use super::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1357;
use super::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1358;
use super::source_ml_source1_temporary_read_unicode_case_mapping_loop1352;
use super::source_ml_source1_temporary_read_unicode_case_mapping_loop1353;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_case_cased_ranges() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![
        65f64, 90f64, 97f64, 122f64, 170f64, 170f64, 181f64, 181f64, 186f64, 186f64, 192f64,
        214f64, 216f64, 246f64, 248f64, 442f64, 444f64, 447f64, 452f64, 659f64, 661f64, 687f64,
        688f64, 696f64, 704f64, 705f64, 736f64, 740f64, 837f64, 837f64, 880f64, 883f64, 886f64,
        887f64, 890f64, 890f64, 891f64, 893f64, 895f64, 895f64, 902f64, 902f64, 904f64, 906f64,
        908f64, 908f64, 910f64, 929f64, 931f64, 1013f64, 1015f64, 1153f64, 1162f64, 1327f64,
        1329f64, 1366f64, 1376f64, 1416f64, 4256f64, 4293f64, 4295f64, 4295f64, 4301f64, 4301f64,
        4304f64, 4346f64, 4348f64, 4348f64, 4349f64, 4351f64, 5024f64, 5109f64, 5112f64, 5117f64,
        7296f64, 7306f64, 7312f64, 7354f64, 7357f64, 7359f64, 7424f64, 7467f64, 7468f64, 7530f64,
        7531f64, 7543f64, 7544f64, 7544f64, 7545f64, 7578f64, 7579f64, 7615f64, 7680f64, 7957f64,
        7960f64, 7965f64, 7968f64, 8005f64, 8008f64, 8013f64, 8016f64, 8023f64, 8025f64, 8025f64,
        8027f64, 8027f64, 8029f64, 8029f64, 8031f64, 8061f64, 8064f64, 8116f64, 8118f64, 8124f64,
        8126f64, 8126f64, 8130f64, 8132f64, 8134f64, 8140f64, 8144f64, 8147f64, 8150f64, 8155f64,
        8160f64, 8172f64, 8178f64, 8180f64, 8182f64, 8188f64, 8305f64, 8305f64, 8319f64, 8319f64,
        8336f64, 8348f64, 8450f64, 8450f64, 8455f64, 8455f64, 8458f64, 8467f64, 8469f64, 8469f64,
        8473f64, 8477f64, 8484f64, 8484f64, 8486f64, 8486f64, 8488f64, 8488f64, 8490f64, 8493f64,
        8495f64, 8500f64, 8505f64, 8505f64, 8508f64, 8511f64, 8517f64, 8521f64, 8526f64, 8526f64,
        8544f64, 8575f64, 8579f64, 8580f64, 9398f64, 9449f64, 11264f64, 11387f64, 11388f64,
        11389f64, 11390f64, 11492f64, 11499f64, 11502f64, 11506f64, 11507f64, 11520f64, 11557f64,
        11559f64, 11559f64, 11565f64, 11565f64, 42560f64, 42605f64, 42624f64, 42651f64, 42652f64,
        42653f64, 42786f64, 42863f64, 42864f64, 42864f64, 42865f64, 42887f64, 42891f64, 42894f64,
        42896f64, 42957f64, 42960f64, 42961f64, 42963f64, 42963f64, 42965f64, 42972f64, 42994f64,
        42996f64, 42997f64, 42998f64, 43000f64, 43001f64, 43002f64, 43002f64, 43824f64, 43866f64,
        43868f64, 43871f64, 43872f64, 43880f64, 43881f64, 43881f64, 43888f64, 43967f64, 64256f64,
        64262f64, 64275f64, 64279f64, 65313f64, 65338f64, 65345f64, 65370f64, 66560f64, 66639f64,
        66736f64, 66771f64, 66776f64, 66811f64, 66928f64, 66938f64, 66940f64, 66954f64, 66956f64,
        66962f64, 66964f64, 66965f64, 66967f64, 66977f64, 66979f64, 66993f64, 66995f64, 67001f64,
        67003f64, 67004f64, 67456f64, 67456f64, 67459f64, 67461f64, 67463f64, 67504f64, 67506f64,
        67514f64, 68736f64, 68786f64, 68800f64, 68850f64, 68944f64, 68965f64, 68976f64, 68997f64,
        71840f64, 71903f64, 93760f64, 93823f64, 119808f64, 119892f64, 119894f64, 119964f64,
        119966f64, 119967f64, 119970f64, 119970f64, 119973f64, 119974f64, 119977f64, 119980f64,
        119982f64, 119993f64, 119995f64, 119995f64, 119997f64, 120003f64, 120005f64, 120069f64,
        120071f64, 120074f64, 120077f64, 120084f64, 120086f64, 120092f64, 120094f64, 120121f64,
        120123f64, 120126f64, 120128f64, 120132f64, 120134f64, 120134f64, 120138f64, 120144f64,
        120146f64, 120485f64, 120488f64, 120512f64, 120514f64, 120538f64, 120540f64, 120570f64,
        120572f64, 120596f64, 120598f64, 120628f64, 120630f64, 120654f64, 120656f64, 120686f64,
        120688f64, 120712f64, 120714f64, 120744f64, 120746f64, 120770f64, 120772f64, 120779f64,
        122624f64, 122633f64, 122635f64, 122654f64, 122661f64, 122666f64, 122928f64, 122989f64,
        125184f64, 125251f64, 127280f64, 127305f64, 127312f64, 127337f64, 127344f64, 127369f64,
    ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_case_ignorable_ranges() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![
        39f64, 39f64, 46f64, 46f64, 58f64, 58f64, 94f64, 94f64, 96f64, 96f64, 168f64, 168f64,
        173f64, 173f64, 175f64, 175f64, 180f64, 180f64, 183f64, 183f64, 184f64, 184f64, 688f64,
        705f64, 706f64, 709f64, 710f64, 721f64, 722f64, 735f64, 736f64, 740f64, 741f64, 747f64,
        748f64, 748f64, 749f64, 749f64, 750f64, 750f64, 751f64, 767f64, 768f64, 879f64, 884f64,
        884f64, 885f64, 885f64, 890f64, 890f64, 900f64, 901f64, 903f64, 903f64, 1155f64, 1159f64,
        1160f64, 1161f64, 1369f64, 1369f64, 1375f64, 1375f64, 1425f64, 1469f64, 1471f64, 1471f64,
        1473f64, 1474f64, 1476f64, 1477f64, 1479f64, 1479f64, 1524f64, 1524f64, 1536f64, 1541f64,
        1552f64, 1562f64, 1564f64, 1564f64, 1600f64, 1600f64, 1611f64, 1631f64, 1648f64, 1648f64,
        1750f64, 1756f64, 1757f64, 1757f64, 1759f64, 1764f64, 1765f64, 1766f64, 1767f64, 1768f64,
        1770f64, 1773f64, 1807f64, 1807f64, 1809f64, 1809f64, 1840f64, 1866f64, 1958f64, 1968f64,
        2027f64, 2035f64, 2036f64, 2037f64, 2042f64, 2042f64, 2045f64, 2045f64, 2070f64, 2073f64,
        2074f64, 2074f64, 2075f64, 2083f64, 2084f64, 2084f64, 2085f64, 2087f64, 2088f64, 2088f64,
        2089f64, 2093f64, 2137f64, 2139f64, 2184f64, 2184f64, 2192f64, 2193f64, 2199f64, 2207f64,
        2249f64, 2249f64, 2250f64, 2273f64, 2274f64, 2274f64, 2275f64, 2306f64, 2362f64, 2362f64,
        2364f64, 2364f64, 2369f64, 2376f64, 2381f64, 2381f64, 2385f64, 2391f64, 2402f64, 2403f64,
        2417f64, 2417f64, 2433f64, 2433f64, 2492f64, 2492f64, 2497f64, 2500f64, 2509f64, 2509f64,
        2530f64, 2531f64, 2558f64, 2558f64, 2561f64, 2562f64, 2620f64, 2620f64, 2625f64, 2626f64,
        2631f64, 2632f64, 2635f64, 2637f64, 2641f64, 2641f64, 2672f64, 2673f64, 2677f64, 2677f64,
        2689f64, 2690f64, 2748f64, 2748f64, 2753f64, 2757f64, 2759f64, 2760f64, 2765f64, 2765f64,
        2786f64, 2787f64, 2810f64, 2815f64, 2817f64, 2817f64, 2876f64, 2876f64, 2879f64, 2879f64,
        2881f64, 2884f64, 2893f64, 2893f64, 2901f64, 2902f64, 2914f64, 2915f64, 2946f64, 2946f64,
        3008f64, 3008f64, 3021f64, 3021f64, 3072f64, 3072f64, 3076f64, 3076f64, 3132f64, 3132f64,
        3134f64, 3136f64, 3142f64, 3144f64, 3146f64, 3149f64, 3157f64, 3158f64, 3170f64, 3171f64,
        3201f64, 3201f64, 3260f64, 3260f64, 3263f64, 3263f64, 3270f64, 3270f64, 3276f64, 3277f64,
        3298f64, 3299f64, 3328f64, 3329f64, 3387f64, 3388f64, 3393f64, 3396f64, 3405f64, 3405f64,
        3426f64, 3427f64, 3457f64, 3457f64, 3530f64, 3530f64, 3538f64, 3540f64, 3542f64, 3542f64,
        3633f64, 3633f64, 3636f64, 3642f64, 3654f64, 3654f64, 3655f64, 3662f64, 3761f64, 3761f64,
        3764f64, 3772f64, 3782f64, 3782f64, 3784f64, 3790f64, 3864f64, 3865f64, 3893f64, 3893f64,
        3895f64, 3895f64, 3897f64, 3897f64, 3953f64, 3966f64, 3968f64, 3972f64, 3974f64, 3975f64,
        3981f64, 3991f64, 3993f64, 4028f64, 4038f64, 4038f64, 4141f64, 4144f64, 4146f64, 4151f64,
        4153f64, 4154f64, 4157f64, 4158f64, 4184f64, 4185f64, 4190f64, 4192f64, 4209f64, 4212f64,
        4226f64, 4226f64, 4229f64, 4230f64, 4237f64, 4237f64, 4253f64, 4253f64, 4348f64, 4348f64,
        4957f64, 4959f64, 5906f64, 5908f64, 5938f64, 5939f64, 5970f64, 5971f64, 6002f64, 6003f64,
        6068f64, 6069f64, 6071f64, 6077f64, 6086f64, 6086f64, 6089f64, 6099f64, 6103f64, 6103f64,
        6109f64, 6109f64, 6155f64, 6157f64, 6158f64, 6158f64, 6159f64, 6159f64, 6211f64, 6211f64,
        6277f64, 6278f64, 6313f64, 6313f64, 6432f64, 6434f64, 6439f64, 6440f64, 6450f64, 6450f64,
        6457f64, 6459f64, 6679f64, 6680f64, 6683f64, 6683f64, 6742f64, 6742f64, 6744f64, 6750f64,
        6752f64, 6752f64, 6754f64, 6754f64, 6757f64, 6764f64, 6771f64, 6780f64, 6783f64, 6783f64,
        6823f64, 6823f64, 6832f64, 6845f64, 6846f64, 6846f64, 6847f64, 6862f64, 6912f64, 6915f64,
        6964f64, 6964f64, 6966f64, 6970f64, 6972f64, 6972f64, 6978f64, 6978f64, 7019f64, 7027f64,
        7040f64, 7041f64, 7074f64, 7077f64, 7080f64, 7081f64, 7083f64, 7085f64, 7142f64, 7142f64,
        7144f64, 7145f64, 7149f64, 7149f64, 7151f64, 7153f64, 7212f64, 7219f64, 7222f64, 7223f64,
        7288f64, 7293f64, 7376f64, 7378f64, 7380f64, 7392f64, 7394f64, 7400f64, 7405f64, 7405f64,
        7412f64, 7412f64, 7416f64, 7417f64, 7468f64, 7530f64, 7544f64, 7544f64, 7579f64, 7615f64,
        7616f64, 7679f64, 8125f64, 8125f64, 8127f64, 8129f64, 8141f64, 8143f64, 8157f64, 8159f64,
        8173f64, 8175f64, 8189f64, 8190f64, 8203f64, 8207f64, 8216f64, 8216f64, 8217f64, 8217f64,
        8228f64, 8228f64, 8231f64, 8231f64, 8234f64, 8238f64, 8288f64, 8292f64, 8294f64, 8303f64,
        8305f64, 8305f64, 8319f64, 8319f64, 8336f64, 8348f64, 8400f64, 8412f64, 8413f64, 8416f64,
        8417f64, 8417f64, 8418f64, 8420f64, 8421f64, 8432f64, 11388f64, 11389f64, 11503f64,
        11505f64, 11631f64, 11631f64, 11647f64, 11647f64, 11744f64, 11775f64, 11823f64, 11823f64,
        12293f64, 12293f64, 12330f64, 12333f64, 12337f64, 12341f64, 12347f64, 12347f64, 12441f64,
        12442f64, 12443f64, 12444f64, 12445f64, 12446f64, 12540f64, 12542f64, 40981f64, 40981f64,
        42232f64, 42237f64, 42508f64, 42508f64, 42607f64, 42607f64, 42608f64, 42610f64, 42612f64,
        42621f64, 42623f64, 42623f64, 42652f64, 42653f64, 42654f64, 42655f64, 42736f64, 42737f64,
        42752f64, 42774f64, 42775f64, 42783f64, 42784f64, 42785f64, 42864f64, 42864f64, 42888f64,
        42888f64, 42889f64, 42890f64, 42994f64, 42996f64, 43000f64, 43001f64, 43010f64, 43010f64,
        43014f64, 43014f64, 43019f64, 43019f64, 43045f64, 43046f64, 43052f64, 43052f64, 43204f64,
        43205f64, 43232f64, 43249f64, 43263f64, 43263f64, 43302f64, 43309f64, 43335f64, 43345f64,
        43392f64, 43394f64, 43443f64, 43443f64, 43446f64, 43449f64, 43452f64, 43453f64, 43471f64,
        43471f64, 43493f64, 43493f64, 43494f64, 43494f64, 43561f64, 43566f64, 43569f64, 43570f64,
        43573f64, 43574f64, 43587f64, 43587f64, 43596f64, 43596f64, 43632f64, 43632f64, 43644f64,
        43644f64, 43696f64, 43696f64, 43698f64, 43700f64, 43703f64, 43704f64, 43710f64, 43711f64,
        43713f64, 43713f64, 43741f64, 43741f64, 43756f64, 43757f64, 43763f64, 43764f64, 43766f64,
        43766f64, 43867f64, 43867f64, 43868f64, 43871f64, 43881f64, 43881f64, 43882f64, 43883f64,
        44005f64, 44005f64, 44008f64, 44008f64, 44013f64, 44013f64, 64286f64, 64286f64, 64434f64,
        64450f64, 65024f64, 65039f64, 65043f64, 65043f64, 65056f64, 65071f64, 65106f64, 65106f64,
        65109f64, 65109f64, 65279f64, 65279f64, 65287f64, 65287f64, 65294f64, 65294f64, 65306f64,
        65306f64, 65342f64, 65342f64, 65344f64, 65344f64, 65392f64, 65392f64, 65438f64, 65439f64,
        65507f64, 65507f64, 65529f64, 65531f64, 66045f64, 66045f64, 66272f64, 66272f64, 66422f64,
        66426f64, 67456f64, 67461f64, 67463f64, 67504f64, 67506f64, 67514f64, 68097f64, 68099f64,
        68101f64, 68102f64, 68108f64, 68111f64, 68152f64, 68154f64, 68159f64, 68159f64, 68325f64,
        68326f64, 68900f64, 68903f64, 68942f64, 68942f64, 68969f64, 68973f64, 68975f64, 68975f64,
        69291f64, 69292f64, 69372f64, 69375f64, 69446f64, 69456f64, 69506f64, 69509f64, 69633f64,
        69633f64, 69688f64, 69702f64, 69744f64, 69744f64, 69747f64, 69748f64, 69759f64, 69761f64,
        69811f64, 69814f64, 69817f64, 69818f64, 69821f64, 69821f64, 69826f64, 69826f64, 69837f64,
        69837f64, 69888f64, 69890f64, 69927f64, 69931f64, 69933f64, 69940f64, 70003f64, 70003f64,
        70016f64, 70017f64, 70070f64, 70078f64, 70089f64, 70092f64, 70095f64, 70095f64, 70191f64,
        70193f64, 70196f64, 70196f64, 70198f64, 70199f64, 70206f64, 70206f64, 70209f64, 70209f64,
        70367f64, 70367f64, 70371f64, 70378f64, 70400f64, 70401f64, 70459f64, 70460f64, 70464f64,
        70464f64, 70502f64, 70508f64, 70512f64, 70516f64, 70587f64, 70592f64, 70606f64, 70606f64,
        70608f64, 70608f64, 70610f64, 70610f64, 70625f64, 70626f64, 70712f64, 70719f64, 70722f64,
        70724f64, 70726f64, 70726f64, 70750f64, 70750f64, 70835f64, 70840f64, 70842f64, 70842f64,
        70847f64, 70848f64, 70850f64, 70851f64, 71090f64, 71093f64, 71100f64, 71101f64, 71103f64,
        71104f64, 71132f64, 71133f64, 71219f64, 71226f64, 71229f64, 71229f64, 71231f64, 71232f64,
        71339f64, 71339f64, 71341f64, 71341f64, 71344f64, 71349f64, 71351f64, 71351f64, 71453f64,
        71453f64, 71455f64, 71455f64, 71458f64, 71461f64, 71463f64, 71467f64, 71727f64, 71735f64,
        71737f64, 71738f64, 71995f64, 71996f64, 71998f64, 71998f64, 72003f64, 72003f64, 72148f64,
        72151f64, 72154f64, 72155f64, 72160f64, 72160f64, 72193f64, 72202f64, 72243f64, 72248f64,
        72251f64, 72254f64, 72263f64, 72263f64, 72273f64, 72278f64, 72281f64, 72283f64, 72330f64,
        72342f64, 72344f64, 72345f64, 72752f64, 72758f64, 72760f64, 72765f64, 72767f64, 72767f64,
        72850f64, 72871f64, 72874f64, 72880f64, 72882f64, 72883f64, 72885f64, 72886f64, 73009f64,
        73014f64, 73018f64, 73018f64, 73020f64, 73021f64, 73023f64, 73029f64, 73031f64, 73031f64,
        73104f64, 73105f64, 73109f64, 73109f64, 73111f64, 73111f64, 73459f64, 73460f64, 73472f64,
        73473f64, 73526f64, 73530f64, 73536f64, 73536f64, 73538f64, 73538f64, 73562f64, 73562f64,
        78896f64, 78911f64, 78912f64, 78912f64, 78919f64, 78933f64, 90398f64, 90409f64, 90413f64,
        90415f64, 92912f64, 92916f64, 92976f64, 92982f64, 92992f64, 92995f64, 93504f64, 93506f64,
        93547f64, 93548f64, 94031f64, 94031f64, 94095f64, 94098f64, 94099f64, 94111f64, 94176f64,
        94177f64, 94179f64, 94179f64, 94180f64, 94180f64, 110576f64, 110579f64, 110581f64,
        110587f64, 110589f64, 110590f64, 113821f64, 113822f64, 113824f64, 113827f64, 118528f64,
        118573f64, 118576f64, 118598f64, 119143f64, 119145f64, 119155f64, 119162f64, 119163f64,
        119170f64, 119173f64, 119179f64, 119210f64, 119213f64, 119362f64, 119364f64, 121344f64,
        121398f64, 121403f64, 121452f64, 121461f64, 121461f64, 121476f64, 121476f64, 121499f64,
        121503f64, 121505f64, 121519f64, 122880f64, 122886f64, 122888f64, 122904f64, 122907f64,
        122913f64, 122915f64, 122916f64, 122918f64, 122922f64, 122928f64, 122989f64, 123023f64,
        123023f64, 123184f64, 123190f64, 123191f64, 123197f64, 123566f64, 123566f64, 123628f64,
        123631f64, 124139f64, 124139f64, 124140f64, 124143f64, 124398f64, 124399f64, 125136f64,
        125142f64, 125252f64, 125258f64, 125259f64, 125259f64, 127995f64, 127999f64, 917505f64,
        917505f64, 917536f64, 917631f64, 917760f64, 917999f64,
    ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unicode_case_property(
    point: f64,
    ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> bool {
    {
        let first = 0f64;
        {
            let last = ({
                let ml_reference_array2030 = ranges.clone();
                let ml_reference_projection2030 = ml_reference_array2030.borrow().len() as f64;
                ml_reference_projection2030
            } / 2f64)
                .floor();
            {
                let source_ml_source2_s32576 =
                    self::source_ml_source1_temporary_accept_unicode_case_property_loop1351(
                        point,
                        ranges.clone(),
                        first,
                        last,
                    );
                match source_ml_source2_s32576.clone() {
                    self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Done => {
                        false
                    }
                    self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Return(source_ml_source1_temporary_result1351) => {
                        source_ml_source1_temporary_result1351
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_case_mapping(
    point: f64,
    ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = 0f64;
        {
            let last = ({
                let ml_reference_array2031 = indexes.clone();
                let ml_reference_projection2031 = ml_reference_array2031.borrow().len() as f64;
                ml_reference_projection2031
            } / 3f64)
                .floor();
            {
                let source_ml_source2_s32577 =
                    self::source_ml_source1_temporary_read_unicode_case_mapping_loop1352(
                        point,
                        indexes.clone(),
                        values.clone(),
                        first,
                        last,
                    );
                match source_ml_source2_s32577.clone() {
                    self::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Done(first_2, last_2) => {
                        {
                            let first_3 = 0f64;
                            {
                                let last_3 = (({ let ml_reference_array2032 = ranges.clone(); let ml_reference_projection2032 = ml_reference_array2032.borrow().len() as f64; ml_reference_projection2032 } / 4f64)).floor();
                                {
                                    let source_ml_source2_s32578 = self::source_ml_source1_temporary_read_unicode_case_mapping_loop1353(point, ranges.clone(), first_3, last_3);
                                    match source_ml_source2_s32578.clone() {
                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Done => {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![point]))
                                        }
                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Return(source_ml_source1_temporary_result1353) => {
                                            source_ml_source1_temporary_result1353.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Return(source_ml_source1_temporary_result1353_2) => {
                        source_ml_source1_temporary_result1353_2.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn map_unicode_case_utf16_columns(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    upper: bool,
    ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    cased: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ignorable: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let points = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let source_ml_source2_s32579 =
                    self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1356(
                        text.clone(),
                        points.clone(),
                        at,
                    );
                match source_ml_source2_s32579.clone() {
                    self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Done => {
                        {
                            let following = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let at_2 = ({ let ml_reference_array2033 = points.clone(); let ml_reference_projection2033 = ml_reference_array2033.borrow().len() as f64; ml_reference_projection2033 } - 1f64);
                                {
                                    let source_ml_source2_s32580 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1357(cased.clone(), ignorable.clone(), points.clone(), following.clone(), false, at_2);
                                    match source_ml_source2_s32580.clone() {
                                        self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Done => {
                                            {
                                                let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let at_3 = 0f64;
                                                    {
                                                        let source_ml_source2_s32581 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1358(upper, ranges.clone(), indexes.clone(), values.clone(), cased.clone(), ignorable.clone(), points.clone(), following.clone(), result.clone(), false, at_3);
                                                        match source_ml_source2_s32581.clone() {
                                                            self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Done => {
                                                                result.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn map_unicode_case_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    upper: bool,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let ranges = if upper {
            self::read_unicode_case_upper_ranges()
        } else {
            self::read_unicode_case_lower_ranges()
        };
        {
            let indexes = if upper {
                self::read_unicode_case_upper_indexes()
            } else {
                self::read_unicode_case_lower_indexes()
            };
            {
                let values = if upper {
                    self::read_unicode_case_upper_values()
                } else {
                    self::read_unicode_case_lower_values()
                };
                self::map_unicode_case_utf16_columns(
                    text.clone(),
                    upper,
                    ranges.clone(),
                    indexes.clone(),
                    values.clone(),
                    self::read_unicode_case_cased_ranges(),
                    self::read_unicode_case_ignorable_ranges(),
                )
            }
        }
    }
}
