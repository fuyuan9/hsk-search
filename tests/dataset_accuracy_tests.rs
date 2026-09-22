use hsk_search::load_dataset;
use hsk_search::search::{search_items, KindFilter, LevelFilter};

#[test]
fn test_embedded_dataset_size_and_integrity() {
    let items = load_dataset().expect("Failed to load embedded HSK 3.0 dataset");
    assert_eq!(items.len(), 14041, "Dataset must contain all 14,041 items");

    let words_count = items
        .iter()
        .filter(|i| i.kind == hsk_search::models::EntryKind::Word)
        .count();
    let hanzi_count = items
        .iter()
        .filter(|i| i.kind == hsk_search::models::EntryKind::Hanzi)
        .count();

    assert_eq!(words_count, 11042, "Must contain exactly 11,042 words");
    assert_eq!(
        hanzi_count, 2999,
        "Must contain exactly 2,999 hanzi characters"
    );
}

#[test]
fn test_polyphone_accuracy_yinhang_vs_xingzou() {
    let items = load_dataset().expect("Failed to load dataset");

    // "银行" must be yínháng / yinhang / yh
    let yinhang = items
        .iter()
        .find(|i| i.simplified == "银行")
        .expect("银行 must exist");
    assert_eq!(yinhang.pinyin_clean, "yinhang");
    assert_eq!(yinhang.pinyin_initials, "yh");
    assert!(yinhang.pinyin_display.contains("háng"));

    // "行走" must be xíngzǒu / xingzou / xz
    let xingzou = items
        .iter()
        .find(|i| i.simplified == "行走")
        .expect("行走 must exist");
    assert_eq!(xingzou.pinyin_clean, "xingzou");
    assert_eq!(xingzou.pinyin_initials, "xz");
    assert!(xingzou.pinyin_display.contains("xíng"));
}

#[test]
fn test_polyphone_accuracy_yinyue_vs_kuaile() {
    let items = load_dataset().expect("Failed to load dataset");

    // "音乐" must be yīnyuè / yinyue / yy
    let yinyue = items
        .iter()
        .find(|i| i.simplified == "音乐")
        .expect("音乐 must exist");
    assert_eq!(yinyue.pinyin_clean, "yinyue");
    assert_eq!(yinyue.pinyin_initials, "yy");

    // "快乐" must be kuàilè / kuaile / kl
    let kuaile = items
        .iter()
        .find(|i| i.simplified == "快乐")
        .expect("快乐 must exist");
    assert_eq!(kuaile.pinyin_clean, "kuaile");
    assert_eq!(kuaile.pinyin_initials, "kl");
}

#[test]
fn test_polyphone_accuracy_zhangda_vs_changcheng() {
    let items = load_dataset().expect("Failed to load dataset");

    // "长大" must be zhǎngdà / zhangda / zd
    let zhangda = items
        .iter()
        .find(|i| i.simplified == "长大")
        .expect("长大 must exist");
    assert_eq!(zhangda.pinyin_clean, "zhangda");
    assert_eq!(zhangda.pinyin_initials, "zd");

    // "长城" must be chángchéng / changcheng / cc
    let changcheng = items
        .iter()
        .find(|i| i.simplified == "长城")
        .expect("长城 must exist");
    assert_eq!(changcheng.pinyin_clean, "changcheng");
    assert_eq!(changcheng.pinyin_initials, "cc");
}

#[test]
fn test_polyphone_accuracy_chongxin_vs_zhongda() {
    let items = load_dataset().expect("Failed to load dataset");

    // "重新" must be chóngxīn / chongxin / cx
    let chongxin = items
        .iter()
        .find(|i| i.simplified == "重新")
        .expect("重新 must exist");
    assert_eq!(chongxin.pinyin_clean, "chongxin");
    assert_eq!(chongxin.pinyin_initials, "cx");

    // "重大" must be zhòngdà / zhongda / zd
    let zhongda = items
        .iter()
        .find(|i| i.simplified == "重大")
        .expect("重大 must exist");
    assert_eq!(zhongda.pinyin_clean, "zhongda");
    assert_eq!(zhongda.pinyin_initials, "zd");
}

#[test]
fn test_polyphone_accuracy_pianyi_vs_bianli() {
    let items = load_dataset().expect("Failed to load dataset");

    // "便宜" must be piányi / pianyi / py
    let pianyi = items
        .iter()
        .find(|i| i.simplified == "便宜")
        .expect("便宜 must exist");
    assert_eq!(pianyi.pinyin_clean, "pianyi");
    assert_eq!(pianyi.pinyin_initials, "py");

    // "便利" must be biànlì / bianli / bl
    let bianli = items
        .iter()
        .find(|i| i.simplified == "便利")
        .expect("便利 must exist");
    assert_eq!(bianli.pinyin_clean, "bianli");
    assert_eq!(bianli.pinyin_initials, "bl");
}

#[test]
fn test_search_real_dataset() {
    let items = load_dataset().expect("Failed to load dataset");

    // Search by initials "yh" -> both "以后" (HSK 1) and "银行" (HSK 2) match exactly
    let yh_res = search_items(&items, "yh", LevelFilter::All, KindFilter::All);
    assert!(!yh_res.is_empty());
    assert!(yh_res.iter().take(5).any(|item| item.simplified == "银行"));
    assert!(yh_res.iter().take(5).any(|item| item.simplified == "以后"));

    // Search by toneless pinyin "yinhang" -> exact top match is "银行"
    let yinhang_res = search_items(&items, "yinhang", LevelFilter::All, KindFilter::All);
    assert!(!yinhang_res.is_empty());
    assert_eq!(yinhang_res[0].simplified, "银行");

    // Search by initials "zg" -> top result should be "中国"
    let zg_res = search_items(&items, "zg", LevelFilter::All, KindFilter::All);
    assert!(!zg_res.is_empty());
    assert_eq!(zg_res[0].simplified, "中国");

    // Search by toneless pinyin "laoshi" -> top result "老师"
    let laoshi_res = search_items(&items, "laoshi", LevelFilter::All, KindFilter::All);
    assert!(!laoshi_res.is_empty());
    assert_eq!(laoshi_res[0].simplified, "老师");

    // Search by English "bank" -> top result "银行"
    let bank_res = search_items(&items, "bank", LevelFilter::All, KindFilter::All);
    assert!(!bank_res.is_empty());
    assert!(bank_res.iter().any(|item| item.simplified == "银行"));

    // Search by Hanzi "中国" -> exact top match
    let zhongguo_res = search_items(&items, "中国", LevelFilter::All, KindFilter::All);
    assert!(!zhongguo_res.is_empty());
    assert_eq!(zhongguo_res[0].simplified, "中国");
}
