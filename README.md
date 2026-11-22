# iz80
iz80を個人的改善とかしてます

## 変更点
- `Machine`の動的ディスパッチを除去
- 8080デコーダーを削除
- IM0・IM2割り込みを実装(まだ実験段階)
- trace系機能を削除
- リファクタリング

## 速さ
色々削った甲斐か本家の33%ぐらい速くなりました

私のマシンで下のコマンドでzexall実行したタイムでの結果です(適当です)(流石にビルド時間は抜いてます)

## コマンド
zexallの実行:
```shell
cargo test --release -- --nocapture --ignored --test zexall
```

Tiny Basic ([cpuville](http://cpuville.com/Kits/Z80-kits-home.html)):
```shell
cargo run --examples cpuville
```

## テスト結果一覧
810d317時点の結果です

### Diagnostics II by Supersoft Associates
未実施

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
dad <b,d,h,sp>................  ERROR **** crc expected:14474ba6 found:14fe4ad4
aluop nn......................  ERROR **** crc expected:9e922f9e found:c55a67ed
aluop <b,c,d,e,h,l,m,a>.......  ERROR **** crc expected:cf762c86 found:bf843d2f
<daa,cma,stc,cmc>.............  ERROR **** crc expected:bb3f030c found:f63791fc
<inr,dcr> a...................  ERROR **** crc expected:adb6460e found:0c6c603c
<inr,dcr> b...................  ERROR **** crc expected:83ed1345 found:67469e8a
<inx,dcx> b...................  ERROR **** crc expected:f79287cd found:d2236336
<inr,dcr> c...................  ERROR **** crc expected:e5f6721b found:80c4ffc6
<inr,dcr> d...................  ERROR **** crc expected:15b5579a found:837cb56e
<inx,dcx> d...................  ERROR **** crc expected:7f4e2501 found:1e93c274
<inr,dcr> e...................  ERROR **** crc expected:cf2ab396 found:1ec575be
<inr,dcr> h...................  ERROR **** crc expected:12b2952c found:27445666
<inx,dcx> h...................  ERROR **** crc expected:9f2b23c0 found:d14bd4be
<inr,dcr> l...................  ERROR **** crc expected:ff57d356 found:83c10f90
<inr,dcr> m...................  ERROR **** crc expected:92e963bd found:5673810f
<inx,dcx> sp..................  ERROR **** crc expected:d5702fab found:c2cd2924
lhld nnnn.....................  ERROR **** crc expected:a9c3d5cb found:96d63c1e
shld nnnn.....................  ERROR **** crc expected:e8864f26 found:47cf4e46
lxi <b,d,h,sp>,nnnn...........  ERROR **** crc expected:fcf46e12 found:8b8ccdb5
ldax <b,d>....................  ERROR **** crc expected:2b821d5f found:d1548f37
mvi <b,c,d,e,h,l,m,a>,nn......  ERROR **** crc expected:eaa72044 found:94b73132
mov <bcdehla>,<bcdehla>.......  ERROR **** crc expected:10b58cee found:c73eeaf0
sta nnnn / lda nnnn...........  ERROR **** crc expected:ed57af72 found:1bba391b
<rlc,rrc,ral,rar>.............  ERROR **** crc expected:e0d89235 found:e2c8edd0
stax <b,d>....................  ERROR **** crc expected:2b0471e9 found:b1f338a3
Tests complete
```
