macro_rules! generate_enum_variants {
    ($enum_name:ident, $($field:ident),*) => {
        #[derive(Clone, Debug)]
        pub enum $enum_name {
            $($field),*
        }
    };
}

generate_enum_variants!(TSTStrategy, QS, QER, QLR, MS, MER, MLR);
generate_enum_variants!(TSBStrategy, QS, QR, M);
generate_enum_variants!(TXTStrategy, QS, QT, MS, MT);
generate_enum_variants!(TXBStrategy, QS, QT, MS, MT);
generate_enum_variants!(TOTStrategy, QS, QR, MS, MR);
generate_enum_variants!(TOSStrategy, QS, QTU, QTD, QRU, QRD, M);
generate_enum_variants!(TOBStrategy, QS, QR, M);
generate_enum_variants!(SSSStrategy, Q, M);
generate_enum_variants!(SXSStrategy, QS, QT, MS, MT);
generate_enum_variants!(SOTStrategy, Q, MS, MTU, MTD, MRU, MRD);
generate_enum_variants!(SOBStrategy, Q, MS, MTU, MTD, MRU, MRD);
generate_enum_variants!(BSTStrategy, Q, MS, MR);
generate_enum_variants!(BSBStrategy, Q, M);
generate_enum_variants!(BXTStrategy, QS, QT, MS, MT);
generate_enum_variants!(BXBStrategy, QS, QT, MS, MT);
generate_enum_variants!(BOTStrategy, Q, MS, MR);
generate_enum_variants!(BOSStrategy, QS, QTU, QTD, QRU, QRD, M);
generate_enum_variants!(BOBStrategy, Q, M);

#[derive(Clone, Debug)]
pub struct Row {
    tst_qs: usize,
    tst_qer: usize,
    tst_qlr: usize,
    tst_ms: usize,
    tst_mer: usize,
    tst_mlr: usize,
    tsb_qs: usize,
    tsb_qr: usize,
    tsb_m: usize,
    txt_qs: usize,
    txt_qt: usize,
    txt_ms: usize,
    txt_mt: usize,
    txb_qs: usize,
    txb_qt: usize,
    txb_ms: usize,
    txb_mt: usize,
    tot_qs: usize,
    tot_qr: usize,
    tot_ms: usize,
    tot_mr: usize,
    tos_qs: usize,
    tos_qtu: usize,
    tos_qtd: usize,
    tos_qru: usize,
    tos_qrd: usize,
    tos_m: usize,
    tob_qs: usize,
    tob_qr: usize,
    tob_m: usize,
    sss_q: usize,
    sss_m: usize,
    sxs_qs: usize,
    sxs_qt: usize,
    sxs_ms: usize,
    sxs_mt: usize,
    sot_q: usize,
    sot_ms: usize,
    sot_mtu: usize,
    sot_mtd: usize,
    sot_mru: usize,
    sot_mrd: usize,
    sob_q: usize,
    sob_ms: usize,
    sob_mtu: usize,
    sob_mtd: usize,
    sob_mru: usize,
    sob_mrd: usize,
    bst_q: usize,
    bst_ms: usize,
    bst_mr: usize,
    bsb_q: usize,
    bsb_m: usize,
    bxt_qs: usize,
    bxt_qt: usize,
    bxt_ms: usize,
    bxt_mt: usize,
    bxb_qs: usize,
    bxb_qt: usize,
    bxb_ms: usize,
    bxb_mt: usize,
    bot_q: usize,
    bot_ms: usize,
    bot_mr: usize,
    bos_qs: usize,
    bos_qtu: usize,
    bos_qtd: usize,
    bos_qru: usize,
    bos_qrd: usize,
    bos_m: usize,
    bob_q: usize,
    bob_m: usize,
    tst: (usize, TSTStrategy, usize, usize, usize),
    tsb: (usize, TSBStrategy, usize, usize, usize),
    txt: (usize, TXTStrategy, usize, usize, usize),
    txb: (usize, TXBStrategy, usize, usize, usize),
    tot: (usize, TOTStrategy, usize, usize, usize),
    tos: (usize, TOSStrategy, usize, usize, usize),
    tob: (usize, TOBStrategy, usize, usize, usize),
    sss: (usize, SSSStrategy, usize, usize, usize),
    sxs: (usize, SXSStrategy, usize, usize, usize),
    sot: (usize, SOTStrategy, usize, usize, usize),
    sob: (usize, SOBStrategy, usize, usize, usize),
    bst: (usize, BSTStrategy, usize, usize, usize),
    bsb: (usize, BSBStrategy, usize, usize, usize),
    bxt: (usize, BXTStrategy, usize, usize, usize),
    bxb: (usize, BXBStrategy, usize, usize, usize),
    bot: (usize, BOTStrategy, usize, usize, usize),
    bos: (usize, BOSStrategy, usize, usize, usize),
    bob: (usize, BOBStrategy, usize, usize, usize),
}

const TST: [usize; 10] = [0, 1, 3, 6, 9, 13, 16, 20, 24, 27];
const TSB: [usize; 10] = [0, 0, 1, 5, 9, 12, 15, 18, 21, 24];
const TXT: [usize; 10] = [0, 0, 1, 5, 9, 11, 14, 17, 20, 23];
const TXB: [usize; 10] = [0, 1, 3, 6, 9, 13, 16, 19, 22, 26];
const TOT: [usize; 10] = [0, 1, 3, 6, 8, 10, 13, 16, 19, 21];
const TOS: [usize; 10] = [0, 1, 3, 5, 7, 9, 11, 13, 16, 18];
const TOB: [usize; 10] = [0, 2, 4, 7, 9, 13, 16, 19, 23, 26];
const SSS: [usize; 10] = [0, 0, 1, 2, 5, 8, 11, 14, 17, 20];
const SXS: [usize; 10] = [0, 0, 1, 2, 5, 8, 10, 13, 16, 19];
const SOT: [usize; 10] = [0, 1, 3, 5, 7, 9, 11, 13, 16, 18];
const SOB: [usize; 10] = [0, 2, 4, 7, 9, 12, 14, 17, 20, 22];
const BST: [usize; 10] = [0, 1, 3, 6, 9, 13, 16, 20, 24, 27];
const BSB: [usize; 10] = [0, 0, 5, 9, 13, 17, 21, 25, 29, 33];
const BXT: [usize; 10] = [0, 1, 3, 6, 9, 13, 16, 19, 22, 26];
const BXB: [usize; 10] = [0, 0, 5, 9, 13, 17, 21, 25, 29, 33];
const BOT: [usize; 10] = [0, 2, 4, 7, 9, 13, 16, 19, 23, 26];
const BOS: [usize; 10] = [0, 2, 4, 7, 9, 12, 14, 17, 20, 22];
const BOB: [usize; 10] = [0, 3, 6, 9, 12, 15, 18, 22, 25, 29];

const fn initial_row(index: usize) -> Row {
    Row {
        tst_qs: TST[index],
        tst_qer: TST[index],
        tst_qlr: TST[index],
        tst_ms: TST[index],
        tst_mer: TST[index],
        tst_mlr: TST[index],
        tsb_qs: TSB[index],
        tsb_qr: TSB[index],
        tsb_m: TSB[index],
        txt_qs: TXT[index],
        txt_qt: TXT[index],
        txt_ms: TXT[index],
        txt_mt: TXT[index],
        txb_qs: TXB[index],
        txb_qt: TXB[index],
        txb_ms: TXB[index],
        txb_mt: TXB[index],
        tot_qs: TOT[index],
        tot_qr: TOT[index],
        tot_ms: TOT[index],
        tot_mr: TOT[index],
        tos_qs: TOS[index],
        tos_qtu: TOS[index],
        tos_qtd: TOS[index],
        tos_qru: TOS[index],
        tos_qrd: TOS[index],
        tos_m: TOS[index],
        tob_qs: TOB[index],
        tob_qr: TOB[index],
        tob_m: TOB[index],
        sss_q: SSS[index],
        sss_m: SSS[index],
        sxs_qs: SXS[index],
        sxs_qt: SXS[index],
        sxs_ms: SXS[index],
        sxs_mt: SXS[index],
        sot_q: SOT[index],
        sot_ms: SOT[index],
        sot_mtu: SOT[index],
        sot_mtd: SOT[index],
        sot_mru: SOT[index],
        sot_mrd: SOT[index],
        sob_q: SOB[index],
        sob_ms: SOB[index],
        sob_mtu: SOB[index],
        sob_mtd: SOB[index],
        sob_mru: SOB[index],
        sob_mrd: SOB[index],
        bst_q: BST[index],
        bst_ms: BST[index],
        bst_mr: BST[index],
        bsb_q: BSB[index],
        bsb_m: BSB[index],
        bxt_qs: BXT[index],
        bxt_qt: BXT[index],
        bxt_ms: BXT[index],
        bxt_mt: BXT[index],
        bxb_qs: BXB[index],
        bxb_qt: BXB[index],
        bxb_ms: BXB[index],
        bxb_mt: BXB[index],
        bot_q: BOT[index],
        bot_ms: BOT[index],
        bot_mr: BOT[index],
        bos_qs: BOS[index],
        bos_qtu: BOS[index],
        bos_qtd: BOS[index],
        bos_qru: BOS[index],
        bos_qrd: BOS[index],
        bos_m: BOS[index],
        bob_q: BOB[index],
        bob_m: BOB[index],
        tst: (0, TSTStrategy::MS, 0, 0, 0),
        tsb: (0, TSBStrategy::M, 0, 0, 0),
        txt: (0, TXTStrategy::MS, 0, 0, 0),
        txb: (0, TXBStrategy::MS, 0, 0, 0),
        tot: (0, TOTStrategy::MS, 0, 0, 0),
        tos: (0, TOSStrategy::M, 0, 0, 0),
        tob: (0, TOBStrategy::M, 0, 0, 0),
        sss: (0, SSSStrategy::M, 0, 0, 0),
        sxs: (0, SXSStrategy::MS, 0, 0, 0),
        sot: (0, SOTStrategy::MS, 0, 0, 0),
        sob: (0, SOBStrategy::MS, 0, 0, 0),
        bst: (0, BSTStrategy::MS, 0, 0, 0),
        bsb: (0, BSBStrategy::M, 0, 0, 0),
        bxt: (0, BXTStrategy::MS, 0, 0, 0),
        bxb: (0, BXBStrategy::MS, 0, 0, 0),
        bot: (0, BOTStrategy::MS, 0, 0, 0),
        bos: (0, BOSStrategy::M, 0, 0, 0),
        bob: (0, BOBStrategy::M, 0, 0, 0),
    }
}

const INITIAL_ROWS: [Row; 10] = [
    initial_row(0),
    initial_row(1),
    initial_row(2),
    initial_row(3),
    initial_row(4),
    initial_row(5),
    initial_row(6),
    initial_row(7),
    initial_row(8),
    initial_row(9),
];

const fn ts(count: usize) -> usize {
    count
}
const fn tt(count: usize) -> usize {
    count
}
const fn tb(count: usize) -> usize {
    count * 2
}
const fn bs(count: usize) -> usize {
    count
}
const fn bt(count: usize) -> usize {
    count * 2
}
const fn bb(count: usize) -> usize {
    count * 3
}
const fn tstb(x: usize, y: usize, z: usize) -> usize {
    ts(x) + tt(y) + tb(z)
}
const fn tsbt(x: usize, y: usize, z: usize) -> usize {
    ts(x) + tb(y) + tt(z)
}
const fn ttsb(x: usize, y: usize, z: usize) -> usize {
    tt(x) + ts(y) + tb(z)
}
const fn ttbs(x: usize, y: usize, z: usize) -> usize {
    tt(x) + tb(y) + ts(z)
}
const fn tbst(x: usize, y: usize, z: usize) -> usize {
    tb(x) + ts(y) + tt(z)
}
const fn tbts(x: usize, y: usize, z: usize) -> usize {
    tb(x) + tt(y) + ts(z)
}
const fn bstb(x: usize, y: usize, z: usize) -> usize {
    bs(x) + bt(y) + bb(z)
}
const fn bsbt(x: usize, y: usize, z: usize) -> usize {
    bs(x) + bb(y) + bt(z)
}
const fn btsb(x: usize, y: usize, z: usize) -> usize {
    bt(x) + bs(y) + bb(z)
}
const fn btbs(x: usize, y: usize, z: usize) -> usize {
    bt(x) + bb(y) + bs(z)
}
const fn bbst(x: usize, y: usize, z: usize) -> usize {
    bb(x) + bs(y) + bt(z)
}
const fn bbts(x: usize, y: usize, z: usize) -> usize {
    bb(x) + bt(y) + bs(z)
}

macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr), *) => {{
        let rest_min = min!($($rest), *);
        if $x < rest_min {
            $x
        } else {
            rest_min
        }
    }};
}

const fn tst(r: &Row) -> usize {
    min!(r.tst_qs, r.tst_qer, r.tst_qlr, r.tst_ms, r.tst_mer, r.tst_mlr)
}
const fn tsb(r: &Row) -> usize {
    min!(r.tsb_qs, r.tsb_qr, r.tsb_m)
}
const fn txt(r: &Row) -> usize {
    min!(r.txt_qs, r.txt_qt, r.txt_ms, r.txt_mt)
}
const fn txb(r: &Row) -> usize {
    min!(r.txb_qs, r.txb_qt, r.txb_ms, r.txb_mt)
}
const fn tot(r: &Row) -> usize {
    min!(r.tot_qs, r.tot_qr, r.tot_ms, r.tot_mr)
}
const fn tos(r: &Row) -> usize {
    min!(r.tos_qs, r.tos_qtu, r.tos_qtd, r.tos_qru, r.tos_qrd, r.tos_m)
}
const fn tob(r: &Row) -> usize {
    min!(r.tob_qs, r.tob_qr, r.tob_m)
}
const fn sss(r: &Row) -> usize {
    min!(r.sss_q, r.sss_m)
}
const fn sxs(r: &Row) -> usize {
    min!(r.sxs_qs, r.sxs_qt, r.sxs_ms, r.sxs_mt)
}
const fn sot(r: &Row) -> usize {
    min!(r.sot_q, r.sot_ms, r.sot_mtu, r.sot_mtd, r.sot_mru, r.sot_mrd)
}
const fn sob(r: &Row) -> usize {
    min!(r.sob_q, r.sob_ms, r.sob_mtu, r.sob_mtd, r.sob_mru, r.sob_mrd)
}
const fn bst(r: &Row) -> usize {
    min!(r.bst_q, r.bst_ms, r.bst_mr)
}
const fn bsb(r: &Row) -> usize {
    min!(r.bsb_q, r.bsb_m)
}
const fn bxt(r: &Row) -> usize {
    min!(r.bxt_qs, r.bxt_qt, r.bxt_ms, r.bxt_mt)
}
const fn bxb(r: &Row) -> usize {
    min!(r.bxb_qs, r.bxb_qt, r.bxb_ms, r.bxb_mt)
}
const fn bot(r: &Row) -> usize {
    min!(r.bot_q, r.bot_ms, r.bot_mr)
}
const fn bos(r: &Row) -> usize {
    min!(r.bos_qs, r.bos_qtu, r.bos_qtd, r.bos_qru, r.bos_qrd, r.bos_m)
}
const fn bob(r: &Row) -> usize {
    min!(r.bob_q, r.bob_m)
}

const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

fn tst_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + tot(&table[z]) + bst(&table[y]) + bot(&table[x])
}
fn tst_qer(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    ttbs(x, y, z) + max(z, y) + tst(&table[z]) + tot(&table[y]) + tot(&table[x])
}
fn tst_qlr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + tot(&table[z]) + max(y, x) + tst(&table[y]) + tot(&table[x])
}
fn tst_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tob(&table[y]) + tsb(&table[z]) + ttbs(x, y, z)
}
fn tst_mer(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tst(&table[y]) + max(y, z) + tot(&table[z]) + tbst(x, y, z)
}
fn tst_mlr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tot(&table[y]) + tst(&table[z]) + max(z, y) + ttbs(x, y, z)
}
fn tsb_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tstb(x, y, z) + bsb(&table[x]) + tob(&table[y]) + bob(&table[z])
}
fn tsb_qr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + max(y, z) + tob(&table[x]) + tsb(&table[y]) + tob(&table[z])
}
fn tsb_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + tot(&table[y]) + tst(&table[z]) + bbts(x, y, z)
}
fn txt_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tsbt(x, y, z) + tot(&table[z]) + sot(&table[y]) + bxt(&table[x])
}
fn txt_qt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tstb(x, y, z) + bot(&table[z]) + sot(&table[y]) + bxt(&table[x])
}
fn txt_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    txb(&table[x]) + tos(&table[y]) + tot(&table[z]) + tsbt(x, y, z)
}
fn txt_mt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    txb(&table[x]) + tos(&table[y]) + tob(&table[z]) + tstb(x, y, z)
}
fn txb_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tstb(x, y, z) + bsb(&table[x]) + tob(&table[y]) + sob(&table[z])
}
fn txb_qt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tsbt(x, y, z) + bsb(&table[x]) + bob(&table[y]) + sob(&table[z])
}
fn txb_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tos(&table[x]) + tot(&table[y]) + tst(&table[z]) + bbts(x, y, z)
}
fn txb_mt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tos(&table[x]) + tob(&table[y]) + tst(&table[z]) + btbs(x, y, z)
}
fn tot_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tsbt(x, y, z) + tst(&table[z]) + bst(&table[y]) + bot(&table[x])
}
fn tot_qr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tsbt(x, y, z) + tst(&table[z]) + max(x, y) + tst(&table[y]) + tot(&table[x])
}
fn tot_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + tsb(&table[y]) + tst(&table[z]) + tsbt(x, y, z)
}
fn tot_mr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tst(&table[y]) + max(y, x) + tst(&table[z]) + tsbt(x, y, z)
}
fn tos_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tstb(x, y, z) + bsb(&table[z]) + tst(&table[y]) + bot(&table[x])
}
fn tos_qtu(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tsbt(x, y, z) + tst(&table[z]) + bst(&table[y]) + bot(&table[x])
}
fn tos_qtd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tsbt(x, y, z) + tsb(&table[z]) + tst(&table[y]) + bot(&table[x])
}
fn tos_qru(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tstb(x, y, z) + bst(&table[z]) + bst(&table[y]) + bot(&table[x])
}
fn tos_qrd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tstb(x, y, z) + tsb(&table[y]) + tsb(&table[z]) + bot(&table[x])
}
fn tos_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    txb(&table[x]) + tos(&table[y]) + tst(&table[z]) + tbst(x, y, z)
}
fn tob_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + bsb(&table[x]) + bob(&table[y]) + tsb(&table[z])
}
fn tob_qr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + max(y, x) + tsb(&table[x]) + tob(&table[y]) + tsb(&table[z])
}
fn tob_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tsb(&table[y]) + tst(&table[z]) + bsbt(x, y, z)
}
fn sss_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    ttbs(x, y, z) + sss(&table[z]) + bot(&table[y]) + tot(&table[x])
}
fn sss_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tob(&table[y]) + sss(&table[z]) + ttbs(x, y, z)
}
fn sxs_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbts(x, y, z) + sss(&table[z]) + tot(&table[y]) + sot(&table[x])
}
fn sxs_qt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    ttbs(x, y, z) + sss(&table[z]) + bot(&table[y]) + sot(&table[x])
}
fn sxs_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tos(&table[x]) + tot(&table[y]) + sss(&table[z]) + tbts(x, y, z)
}
fn sxs_mt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tos(&table[x]) + tob(&table[y]) + sss(&table[z]) + ttbs(x, y, z)
}
fn sot_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + tst(&table[z]) + sot(&table[y]) + bxt(&table[x])
}
fn sot_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + tst(&table[y]) + bsb(&table[z]) + tstb(x, y, z)
}
fn sot_mtu(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + tst(&table[y]) + bst(&table[z]) + tsbt(x, y, z)
}
fn sot_mtd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + tsb(&table[y]) + tst(&table[z]) + tsbt(x, y, z)
}
fn sot_mru(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + bst(&table[y]) + bst(&table[z]) + tsbt(x, y, z)
}
fn sot_mrd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tob(&table[x]) + tsb(&table[y]) + tsb(&table[z]) + tstb(x, y, z)
}
fn sob_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tbst(x, y, z) + bsb(&table[x]) + sob(&table[y]) + txb(&table[z])
}
fn sob_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tst(&table[y]) + bsb(&table[z]) + bstb(x, y, z)
}
fn sob_mtu(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tst(&table[y]) + bst(&table[z]) + bsbt(x, y, z)
}
fn sob_mtd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tsb(&table[y]) + tst(&table[z]) + bsbt(x, y, z)
}
fn sob_mru(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + bst(&table[y]) + bst(&table[z]) + bsbt(x, y, z)
}
fn sob_mrd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    tot(&table[x]) + tsb(&table[y]) + tsb(&table[z]) + bstb(x, y, z)
}
fn bst_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    btbs(x, y, z) + tst(&table[z]) + bot(&table[y]) + tot(&table[x])
}
fn bst_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bot(&table[x]) + bob(&table[y]) + bsb(&table[z]) + ttbs(x, y, z)
}
fn bst_mr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bot(&table[x]) + bot(&table[y]) + bst(&table[z]) + max(z, y) + ttbs(x, y, z)
}
fn bsb_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bstb(x, y, z) + tsb(&table[x]) + tob(&table[y]) + bob(&table[z])
}
fn bsb_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bst(&table[x]) + bot(&table[y]) + bob(&table[z]) + bstb(x, y, z)
}
fn bxt_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bbts(x, y, z) + tst(&table[z]) + tot(&table[y]) + sot(&table[x])
}
fn bxt_qt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    btbs(x, y, z) + tst(&table[z]) + bot(&table[y]) + sot(&table[x])
}
fn bxt_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bos(&table[x]) + bot(&table[y]) + bsb(&table[z]) + tbts(x, y, z)
}
fn bxt_mt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bos(&table[x]) + bob(&table[y]) + bsb(&table[z]) + ttbs(x, y, z)
}
fn bxb_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bstb(x, y, z) + tsb(&table[x]) + tob(&table[y]) + sob(&table[z])
}
fn bxb_qt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bsbt(x, y, z) + tsb(&table[x]) + bob(&table[y]) + sob(&table[z])
}
fn bxb_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bxt(&table[x]) + bos(&table[y]) + bob(&table[z]) + bstb(x, y, z)
}
fn bxb_mt(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bxt(&table[x]) + bos(&table[y]) + bot(&table[z]) + bsbt(x, y, z)
}
fn bot_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bsbt(x, y, z) + tst(&table[z]) + bst(&table[y]) + tot(&table[x])
}
fn bot_ms(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bst(&table[x]) + bob(&table[y]) + bsb(&table[z]) + ttsb(x, y, z)
}
fn bot_mr(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bst(&table[x]) + bot(&table[y]) + bst(&table[z]) + max(z, y) + ttsb(x, y, z)
}
fn bos_qs(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bstb(x, y, z) + bsb(&table[z]) + tst(&table[y]) + tot(&table[x])
}
fn bos_qtu(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bsbt(x, y, z) + tst(&table[z]) + bst(&table[y]) + tot(&table[x])
}
fn bos_qtd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bsbt(x, y, z) + tsb(&table[z]) + tst(&table[y]) + tot(&table[x])
}
fn bos_qru(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bstb(x, y, z) + bst(&table[z]) + bst(&table[y]) + tot(&table[x])
}
fn bos_qrd(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bstb(x, y, z) + tsb(&table[y]) + tsb(&table[z]) + tot(&table[x])
}
fn bos_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bxt(&table[x]) + bos(&table[y]) + bsb(&table[z]) + ttsb(x, y, z)
}
fn bob_q(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bbst(x, y, z) + bsb(&table[x]) + tob(&table[y]) + tsb(&table[z])
}
fn bob_m(table: &Vec<Row>, x: usize, y: usize, z: usize) -> usize {
    bst(&table[x]) + bot(&table[y]) + bsb(&table[z]) + btsb(x, y, z)
}

fn min(to: &mut usize, value: usize) -> bool {
    if *to > value {
        *to = value;
        true
    } else {
        false
    }
}

pub fn table(size: usize) -> Vec<Row> {
    let mut table = INITIAL_ROWS.to_vec();
    for _ in 0..=size {
        let current = table.len();
        let mut row = Row {
            tst_qs: usize::MAX,
            tst_qer: usize::MAX,
            tst_qlr: usize::MAX,
            tst_ms: usize::MAX,
            tst_mer: usize::MAX,
            tst_mlr: usize::MAX,
            tsb_qs: usize::MAX,
            tsb_qr: usize::MAX,
            tsb_m: usize::MAX,
            txt_qs: usize::MAX,
            txt_qt: usize::MAX,
            txt_ms: usize::MAX,
            txt_mt: usize::MAX,
            txb_qs: usize::MAX,
            txb_qt: usize::MAX,
            txb_ms: usize::MAX,
            txb_mt: usize::MAX,
            tot_qs: usize::MAX,
            tot_qr: usize::MAX,
            tot_ms: usize::MAX,
            tot_mr: usize::MAX,
            tos_qs: usize::MAX,
            tos_qtu: usize::MAX,
            tos_qtd: usize::MAX,
            tos_qru: usize::MAX,
            tos_qrd: usize::MAX,
            tos_m: usize::MAX,
            tob_qs: usize::MAX,
            tob_qr: usize::MAX,
            tob_m: usize::MAX,
            sss_q: usize::MAX,
            sss_m: usize::MAX,
            sxs_qs: usize::MAX,
            sxs_qt: usize::MAX,
            sxs_ms: usize::MAX,
            sxs_mt: usize::MAX,
            sot_q: usize::MAX,
            sot_ms: usize::MAX,
            sot_mtu: usize::MAX,
            sot_mtd: usize::MAX,
            sot_mru: usize::MAX,
            sot_mrd: usize::MAX,
            sob_q: usize::MAX,
            sob_ms: usize::MAX,
            sob_mtu: usize::MAX,
            sob_mtd: usize::MAX,
            sob_mru: usize::MAX,
            sob_mrd: usize::MAX,
            bst_q: usize::MAX,
            bst_ms: usize::MAX,
            bst_mr: usize::MAX,
            bsb_q: usize::MAX,
            bsb_m: usize::MAX,
            bxt_qs: usize::MAX,
            bxt_qt: usize::MAX,
            bxt_ms: usize::MAX,
            bxt_mt: usize::MAX,
            bxb_qs: usize::MAX,
            bxb_qt: usize::MAX,
            bxb_ms: usize::MAX,
            bxb_mt: usize::MAX,
            bot_q: usize::MAX,
            bot_ms: usize::MAX,
            bot_mr: usize::MAX,
            bos_qs: usize::MAX,
            bos_qtu: usize::MAX,
            bos_qtd: usize::MAX,
            bos_qru: usize::MAX,
            bos_qrd: usize::MAX,
            bos_m: usize::MAX,
            bob_q: usize::MAX,
            bob_m: usize::MAX,
            tst: (0, TSTStrategy::MS, 0, 0, 0),
            tsb: (0, TSBStrategy::M, 0, 0, 0),
            txt: (0, TXTStrategy::MS, 0, 0, 0),
            txb: (0, TXBStrategy::MS, 0, 0, 0),
            tot: (0, TOTStrategy::MS, 0, 0, 0),
            tos: (0, TOSStrategy::M, 0, 0, 0),
            tob: (0, TOBStrategy::M, 0, 0, 0),
            sss: (0, SSSStrategy::M, 0, 0, 0),
            sxs: (0, SXSStrategy::MS, 0, 0, 0),
            sot: (0, SOTStrategy::MS, 0, 0, 0),
            sob: (0, SOBStrategy::MS, 0, 0, 0),
            bst: (0, BSTStrategy::MS, 0, 0, 0),
            bsb: (0, BSBStrategy::M, 0, 0, 0),
            bxt: (0, BXTStrategy::MS, 0, 0, 0),
            bxb: (0, BXBStrategy::MS, 0, 0, 0),
            bot: (0, BOTStrategy::MS, 0, 0, 0),
            bos: (0, BOSStrategy::M, 0, 0, 0),
            bob: (0, BOBStrategy::M, 0, 0, 0),
        };
        for x in 0..=current {
            for y in 0..=(current - x) {
                let z = current - x - y;
                if x == current || y == current || z == current {
                    continue;
                }
                if min(&mut row.tst_qs, tst_qs(&table, x, y, z)) {
                    row.tst = (row.tst_qs, TSTStrategy::QS, x, y, z);
                }
                if min(&mut row.tst_qer, tst_qer(&table, x, y, z)) {
                    row.tst = (row.tst_qer, TSTStrategy::QER, x, y, z);
                }
                if min(&mut row.tst_qlr, tst_qlr(&table, x, y, z)) {
                    row.tst = (row.tst_qlr, TSTStrategy::QLR, x, y, z);
                }
                if min(&mut row.tst_ms, tst_ms(&table, x, y, z)) {
                    row.tst = (row.tst_ms, TSTStrategy::MS, x, y, z);
                }
                if min(&mut row.tst_mer, tst_mer(&table, x, y, z)) {
                    row.tst = (row.tst_mer, TSTStrategy::MER, x, y, z);
                }
                if min(&mut row.tst_mlr, tst_mlr(&table, x, y, z)) {
                    row.tst = (row.tst_mlr, TSTStrategy::MLR, x, y, z);
                }
                if min(&mut row.tsb_qs, tsb_qs(&table, x, y, z)) {
                    row.tsb = (row.tsb_qs, TSBStrategy::QS, x, y, z);
                }
                if min(&mut row.tsb_qr, tsb_qr(&table, x, y, z)) {
                    row.tsb = (row.tsb_qr, TSBStrategy::QR, x, y, z);
                }
                if min(&mut row.tsb_m, tsb_m(&table, x, y, z)) {
                    row.tsb = (row.tsb_m, TSBStrategy::M, x, y, z);
                }
                if min(&mut row.txt_qs, txt_qs(&table, x, y, z)) {
                    row.txt = (row.txt_qs, TXTStrategy::QS, x, y, z);
                }
                if min(&mut row.txt_qt, txt_qt(&table, x, y, z)) {
                    row.txt = (row.txt_qt, TXTStrategy::QT, x, y, z);
                }
                if min(&mut row.txt_ms, txt_ms(&table, x, y, z)) {
                    row.txt = (row.txt_ms, TXTStrategy::MS, x, y, z);
                }
                if min(&mut row.txt_mt, txt_mt(&table, x, y, z)) {
                    row.txt = (row.txt_mt, TXTStrategy::MT, x, y, z);
                }
                if min(&mut row.txb_qs, txb_qs(&table, x, y, z)) {
                    row.txb = (row.txb_qs, TXBStrategy::QS, x, y, z);
                }
                if min(&mut row.txb_qt, txb_qt(&table, x, y, z)) {
                    row.txb = (row.txb_qt, TXBStrategy::QT, x, y, z);
                }
                if min(&mut row.txb_ms, txb_ms(&table, x, y, z)) {
                    row.txb = (row.txb_ms, TXBStrategy::MS, x, y, z);
                }
                if min(&mut row.txb_mt, txb_mt(&table, x, y, z)) {
                    row.txb = (row.txb_mt, TXBStrategy::MT, x, y, z);
                }
                if min(&mut row.tot_qs, tot_qs(&table, x, y, z)) {
                    row.tot = (row.tot_qs, TOTStrategy::QS, x, y, z);
                }
                if min(&mut row.tot_qr, tot_qr(&table, x, y, z)) {
                    row.tot = (row.tot_qr, TOTStrategy::QR, x, y, z);
                }
                if min(&mut row.tot_ms, tot_ms(&table, x, y, z)) {
                    row.tot = (row.tot_ms, TOTStrategy::MS, x, y, z);
                }
                if min(&mut row.tot_mr, tot_mr(&table, x, y, z)) {
                    row.tot = (row.tot_mr, TOTStrategy::MR, x, y, z);
                }
                if min(&mut row.tos_qs, tos_qs(&table, x, y, z)) {
                    row.tos = (row.tos_qs, TOSStrategy::QS, x, y, z);
                }
                if min(&mut row.tos_qtu, tos_qtu(&table, x, y, z)) {
                    row.tos = (row.tos_qtu, TOSStrategy::QTU, x, y, z);
                }
                if min(&mut row.tos_qtd, tos_qtd(&table, x, y, z)) {
                    row.tos = (row.tos_qtd, TOSStrategy::QTD, x, y, z);
                }
                if min(&mut row.tos_qru, tos_qru(&table, x, y, z)) {
                    row.tos = (row.tos_qru, TOSStrategy::QRU, x, y, z);
                }
                if min(&mut row.tos_qrd, tos_qrd(&table, x, y, z)) {
                    row.tos = (row.tos_qrd, TOSStrategy::QRD, x, y, z);
                }
                if min(&mut row.tos_m, tos_m(&table, x, y, z)) {
                    row.tos = (row.tos_m, TOSStrategy::M, x, y, z);
                }
                if min(&mut row.tob_qs, tob_qs(&table, x, y, z)) {
                    row.tob = (row.tob_qs, TOBStrategy::QS, x, y, z);
                }
                if min(&mut row.tob_qr, tob_qr(&table, x, y, z)) {
                    row.tob = (row.tob_qr, TOBStrategy::QR, x, y, z);
                }
                if min(&mut row.tob_m, tob_m(&table, x, y, z)) {
                    row.tob = (row.tob_m, TOBStrategy::M, x, y, z);
                }
                if min(&mut row.sss_q, sss_q(&table, x, y, z)) {
                    row.sss = (row.sss_q, SSSStrategy::Q, x, y, z);
                }
                if min(&mut row.sss_m, sss_m(&table, x, y, z)) {
                    row.sss = (row.sss_m, SSSStrategy::M, x, y, z);
                }
                if min(&mut row.sxs_qs, sxs_qs(&table, x, y, z)) {
                    row.sxs = (row.sxs_qs, SXSStrategy::QS, x, y, z);
                }
                if min(&mut row.sxs_qt, sxs_qt(&table, x, y, z)) {
                    row.sxs = (row.sxs_qt, SXSStrategy::QT, x, y, z);
                }
                if min(&mut row.sxs_ms, sxs_ms(&table, x, y, z)) {
                    row.sxs = (row.sxs_ms, SXSStrategy::MS, x, y, z);
                }
                if min(&mut row.sxs_mt, sxs_mt(&table, x, y, z)) {
                    row.sxs = (row.sxs_mt, SXSStrategy::MT, x, y, z);
                }
                if min(&mut row.sot_q, sot_q(&table, x, y, z)) {
                    row.sot = (row.sot_q, SOTStrategy::Q, x, y, z);
                }
                if min(&mut row.sot_ms, sot_ms(&table, x, y, z)) {
                    row.sot = (row.sot_ms, SOTStrategy::MS, x, y, z);
                }
                if min(&mut row.sot_mtu, sot_mtu(&table, x, y, z)) {
                    row.sot = (row.sot_mtu, SOTStrategy::MTU, x, y, z);
                }
                if min(&mut row.sot_mtd, sot_mtd(&table, x, y, z)) {
                    row.sot = (row.sot_mtd, SOTStrategy::MTD, x, y, z);
                }
                if min(&mut row.sot_mru, sot_mru(&table, x, y, z)) {
                    row.sot = (row.sot_mru, SOTStrategy::MRU, x, y, z);
                }
                if min(&mut row.sot_mrd, sot_mrd(&table, x, y, z)) {
                    row.sot = (row.sot_mrd, SOTStrategy::MRD, x, y, z);
                }
                if min(&mut row.sob_q, sob_q(&table, x, y, z)) {
                    row.sob = (row.sob_q, SOBStrategy::Q, x, y, z);
                }
                if min(&mut row.sob_ms, sob_ms(&table, x, y, z)) {
                    row.sob = (row.sob_ms, SOBStrategy::MS, x, y, z);
                }
                if min(&mut row.sob_mtu, sob_mtu(&table, x, y, z)) {
                    row.sob = (row.sob_mtu, SOBStrategy::MTU, x, y, z);
                }
                if min(&mut row.sob_mtd, sob_mtd(&table, x, y, z)) {
                    row.sob = (row.sob_mtd, SOBStrategy::MTD, x, y, z);
                }
                if min(&mut row.sob_mru, sob_mru(&table, x, y, z)) {
                    row.sob = (row.sob_mru, SOBStrategy::MRU, x, y, z);
                }
                if min(&mut row.sob_mrd, sob_mrd(&table, x, y, z)) {
                    row.sob = (row.sob_mrd, SOBStrategy::MRD, x, y, z);
                }
                if min(&mut row.bst_q, bst_q(&table, x, y, z)) {
                    row.bst = (row.bst_q, BSTStrategy::Q, x, y, z);
                }
                if min(&mut row.bst_ms, bst_ms(&table, x, y, z)) {
                    row.bst = (row.bst_ms, BSTStrategy::MS, x, y, z);
                }
                if min(&mut row.bst_mr, bst_mr(&table, x, y, z)) {
                    row.bst = (row.bst_mr, BSTStrategy::MR, x, y, z);
                }
                if min(&mut row.bsb_q, bsb_q(&table, x, y, z)) {
                    row.bsb = (row.bsb_q, BSBStrategy::Q, x, y, z);
                }
                if min(&mut row.bsb_m, bsb_m(&table, x, y, z)) {
                    row.bsb = (row.bsb_m, BSBStrategy::M, x, y, z);
                }
                if min(&mut row.bxt_qs, bxt_qs(&table, x, y, z)) {
                    row.bxt = (row.bxt_qs, BXTStrategy::QS, x, y, z);
                }
                if min(&mut row.bxt_qt, bxt_qt(&table, x, y, z)) {
                    row.bxt = (row.bxt_qt, BXTStrategy::QT, x, y, z);
                }
                if min(&mut row.bxt_ms, bxt_ms(&table, x, y, z)) {
                    row.bxt = (row.bxt_ms, BXTStrategy::MS, x, y, z);
                }
                if min(&mut row.bxt_mt, bxt_mt(&table, x, y, z)) {
                    row.bxt = (row.bxt_mt, BXTStrategy::MT, x, y, z);
                }
                if min(&mut row.bxb_qs, bxb_qs(&table, x, y, z)) {
                    row.bxb = (row.bxb_qs, BXBStrategy::QS, x, y, z);
                }
                if min(&mut row.bxb_qt, bxb_qt(&table, x, y, z)) {
                    row.bxb = (row.bxb_qt, BXBStrategy::QT, x, y, z);
                }
                if min(&mut row.bxb_ms, bxb_ms(&table, x, y, z)) {
                    row.bxb = (row.bxb_ms, BXBStrategy::MS, x, y, z);
                }
                if min(&mut row.bxb_mt, bxb_mt(&table, x, y, z)) {
                    row.bxb = (row.bxb_mt, BXBStrategy::MT, x, y, z);
                }
                if min(&mut row.bot_q, bot_q(&table, x, y, z)) {
                    row.bot = (row.bot_q, BOTStrategy::Q, x, y, z);
                }
                if min(&mut row.bot_ms, bot_ms(&table, x, y, z)) {
                    row.bot = (row.bot_ms, BOTStrategy::MS, x, y, z);
                }
                if min(&mut row.bot_mr, bot_mr(&table, x, y, z)) {
                    row.bot = (row.bot_mr, BOTStrategy::MR, x, y, z);
                }
                if min(&mut row.bos_qs, bos_qs(&table, x, y, z)) {
                    row.bos = (row.bos_qs, BOSStrategy::QS, x, y, z);
                }
                if min(&mut row.bos_qtu, bos_qtu(&table, x, y, z)) {
                    row.bos = (row.bos_qtu, BOSStrategy::QTU, x, y, z);
                }
                if min(&mut row.bos_qtd, bos_qtd(&table, x, y, z)) {
                    row.bos = (row.bos_qtd, BOSStrategy::QTD, x, y, z);
                }
                if min(&mut row.bos_qru, bos_qru(&table, x, y, z)) {
                    row.bos = (row.bos_qru, BOSStrategy::QRU, x, y, z);
                }
                if min(&mut row.bos_qrd, bos_qrd(&table, x, y, z)) {
                    row.bos = (row.bos_qrd, BOSStrategy::QRD, x, y, z);
                }
                if min(&mut row.bos_m, bos_m(&table, x, y, z)) {
                    row.bos = (row.bos_m, BOSStrategy::M, x, y, z);
                }
                if min(&mut row.bob_q, bob_q(&table, x, y, z)) {
                    row.bob = (row.bob_q, BOBStrategy::Q, x, y, z);
                }
                if min(&mut row.bob_m, bob_m(&table, x, y, z)) {
                    row.bob = (row.bob_m, BOBStrategy::M, x, y, z);
                }
            }
        }
        table.push(row);
    }
    table
}
