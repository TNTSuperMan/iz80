# iz80
iz80を個人的改善とかしてます

## 変更点
- `Machine`の動的ディスパッチを除去
- 8080デコーダーを削除
- IM0・IM2割り込みを実装(まだ実験段階)
- trace系機能を削除
- リファクタリング

## コマンド
zexallの実行:
```shell
cargo test --release -- --nocapture --ignored --test zexall
```

ex8080の実行(8080は消したので失敗します):
```shell
cargo test --release -- --nocapture --ignored --test ex8080
```

Tiny Basic ([cpuville](http://cpuville.com/Kits/Z80-kits-home.html)):
```shell
cargo run --examples cpuville
```

## 使い方
todo: あとで書く

## テスト結果一覧
todo: 更新して

### Diagnostics II by Supersoft Associates for Intel 8080

```
DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS 8080/8085
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK
```
### Diagnostics II by Supersoft Associates for Z80

```
DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS Z80
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK
```

### Z80 instruction exerciser ZEXALL

```
Z80 instruction exerciser
<adc,sbc> hl,<bc,de,hl,sp>....  OK
add hl,<bc,de,hl,sp>..........  OK
add ix,<bc,de,ix,sp>..........  OK
add iy,<bc,de,iy,sp>..........  OK
aluop a,nn....................  OK
aluop a,<b,c,d,e,h,l,(hl),a>..  OK
aluop a,<ixh,ixl,iyh,iyl>.....  OK
aluop a,(<ix,iy>+1)...........  OK
bit n,(<ix,iy>+1).............  OK
bit n,<b,c,d,e,h,l,(hl),a>....  OK
cpd<r>........................  OK
cpi<r>........................  OK
<daa,cpl,scf,ccf>.............  OK
<inc,dec> a...................  OK
<inc,dec> b...................  OK
<inc,dec> bc..................  OK
<inc,dec> c...................  OK
<inc,dec> d...................  OK
<inc,dec> de..................  OK
<inc,dec> e...................  OK
<inc,dec> h...................  OK
<inc,dec> hl..................  OK
<inc,dec> ix..................  OK
<inc,dec> iy..................  OK
<inc,dec> l...................  OK
<inc,dec> (hl)................  OK
<inc,dec> sp..................  OK
<inc,dec> (<ix,iy>+1).........  OK
<inc,dec> ixh.................  OK
<inc,dec> ixl.................  OK
<inc,dec> iyh.................  OK
<inc,dec> iyl.................  OK
ld <bc,de>,(nnnn).............  OK
ld hl,(nnnn)..................  OK
ld sp,(nnnn)..................  OK
ld <ix,iy>,(nnnn).............  OK
ld (nnnn),<bc,de>.............  OK
ld (nnnn),hl..................  OK
ld (nnnn),sp..................  OK
ld (nnnn),<ix,iy>.............  OK
ld <bc,de,hl,sp>,nnnn.........  OK
ld <ix,iy>,nnnn...............  OK
ld a,<(bc),(de)>..............  OK
ld <b,c,d,e,h,l,(hl),a>,nn....  OK
ld (<ix,iy>+1),nn.............  OK
ld <b,c,d,e>,(<ix,iy>+1)......  OK
ld <h,l>,(<ix,iy>+1)..........  OK
ld a,(<ix,iy>+1)..............  OK
ld <ixh,ixl,iyh,iyl>,nn.......  OK
ld <bcdehla>,<bcdehla>........  OK
ld <bcdexya>,<bcdexya>........  OK
ld a,(nnnn) / ld (nnnn),a.....  OK
ldd<r> (1)....................  OK
ldd<r> (2)....................  OK
ldi<r> (1)....................  OK
ldi<r> (2)....................  OK
neg...........................  OK
<rrd,rld>.....................  OK
<rlca,rrca,rla,rra>...........  OK
shf/rot (<ix,iy>+1)...........  OK
shf/rot <b,c,d,e,h,l,(hl),a>..  OK
<set,res> n,<bcdehl(hl)a>.....  OK
<set,res> n,(<ix,iy>+1).......  OK
ld (<ix,iy>+1),<b,c,d,e>......  OK
ld (<ix,iy>+1),<h,l>..........  OK
ld (<ix,iy>+1),a..............  OK
ld (<bc,de>),a................  OK
Tests complete
```

### 8080 instruction exerciser

```
8080 instruction exerciser (KR580VM80A CPU)
dad <b,d,h,sp>................  OK
aluop nn......................  OK
aluop <b,c,d,e,h,l,m,a>.......  OK
<daa,cma,stc,cmc>.............  OK
<inr,dcr> a...................  OK
<inr,dcr> b...................  OK
<inx,dcx> b...................  OK
<inr,dcr> c...................  OK
<inr,dcr> d...................  OK
<inx,dcx> d...................  OK
<inr,dcr> e...................  OK
<inr,dcr> h...................  OK
<inx,dcx> h...................  OK
<inr,dcr> l...................  OK
<inr,dcr> m...................  OK
<inx,dcx> sp..................  OK
lhld nnnn.....................  OK
shld nnnn.....................  OK
lxi <b,d,h,sp>,nnnn...........  OK
ldax <b,d>....................  OK
mvi <b,c,d,e,h,l,m,a>,nn......  OK
mov <bcdehla>,<bcdehla>.......  OK
sta nnnn / lda nnnn...........  OK
<rlc,rrc,ral,rar>.............  OK
stax <b,d>....................  OK
Tests complete
```
