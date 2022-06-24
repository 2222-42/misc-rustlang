# what to talk

そもそもの疑問は、Iteratorって何だよ、とわからなくなったこと。

ユーザー定義型に対してイテレータを実装することで、なんかそれっぽく理解できるようになりたい。

# Binary Tree

ここで取り扱う二分木(Binary Tree)は、次のように実装するとする。

まず、構造に関しては次の通りである:

- 空(Empty)であるか、もしくは、空ではなくノードを有しており
- 各ノードに、要素と、左枝と右枝があり、左枝と右枝もまたBinaryTreeであり、
- ノードの要素の型は木全体で一致する

。そして、二分木に要素を追加するところに関するルールとして、

1. 木が空であったら、要素が1つあるだけのノードを持つ木を作り
2. 木が空でない場合、追加する値をノードの要素の値と大小を比較し、
    1. 追加する値がノードの要素の値以下であれば、左の枝に追加する先を探しに行き
    2. 追加する値がノードの要素の値より大きいければ、右の枝に追加する先を探しに行く

。また、二分木には特定の型のものを入れるのではなく、大小比較ができるのであればなんでも入れたい。要素の型については比較可能であるという条件をつけたジェネリック構造体としたい。

以上のような二分木を図にすると以下のようになる。

<!-- TODO:（画像） -->

以上をRustのコードにすると以下のようになる。

```rust
#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}
```

### 補足

ジェネリック構造体という用語が出たが、これは、構造体の定義がテンプレートになっていて、任意の型をプラグインできることを意味する。

また、`impl<T: Ord> BinaryTree<T> {...`という記述があったが、これは、

- これはトレイト`Ord`を実装した任意の型`T`について、
- ジェネリック構造体`BinaryTree<T>`のメソッドを`{...}`内で定義すること

を意味する。(なお、Rustにおけるメソッドは構造体定義の中で書くのではなく、別の`impl`ブロックに書く。)

型パラメータ`<T: Ord>`という記法によって、`BinaryTree`の`add`の中において、`Ord`を実装したなんらかの型`T`だ、ということが言えるようになる。

トレイト(Trait)、という用語を使っていたが、これは、

なお、トレイトもジェネリクスも多相性(polymorphism)をRustで実現しているものである。どう違うか、どのように使い分けるかは本発表では扱わない。

また、`Box`という語句もあったが、

構造体やフィールドに`pub`をつけていたが、Rustはデフォルトで非公開にしているので、外部からアクセスするために付け加えた。

## What is expected in implemting Iterator on BinaryTree

さて、このような二分木に対して、`for` 文を回した時、どのように値が取り出されることが期待されるか。

追加の仕方を見る限り、値が小さい順に出されて欲しい、つまり、左下にあるノードの要素から、順次、上に行き、右枝に行き、右枝でも左下から同じ作業をし、それが尽きたら、上に行く、ということを繰り返していく。

そういったスタックを実装していきたい。画像にすると以下のようになる。

<!-- TODO:（画像） -->

# Before implementing 

イテレータってなに？　値の列を生成する値のこと

Rustでの「イテレータ」はトレイト`Iterator`を実装した任意の型のことであり、「イテレート可能」とはトレイト`IntoIterator`を実装した任意の型のこと。これら2つを実装すれば、自作の型に対して、`for`文を回せるようになる。

用語について若干補足する。

- イテレータが生成する値を「アイテム」と呼ぶ。
- アイテムを受け取るコードを「消費者」と呼ぶ。
- 「アダプタ」は、イテレータを消費し、なんらかの有用な動作を受け取って、別のイテレータを作るものであり、`map`や`fold`とかもアダプタとされる。

# Let's implement

我々が目指しているのは、`for`文をぐるぐる回したら、値が最初から最後まで取れて、終了させられることである。実際にそれを使うのは、おそらくすでにイテレータが実装されている型であれば、あまり考えずに使うことができる。例えば、`v`というベクタがあって、そのアイテムをプリントする関数を書こうと思ったら次のように書ける。

```rust
for element in &v {
    println("{}", element)
}
```

しかし、これが実際にやっていることは次のとおりである。

```rust
let mut iterator = (&v).into_iter();

while let Some(element) = iterator.next() {
    println("{}", element)
}
```

つまり、ベクタをイテレータにし、それに対して`next()`を呼んで、アイテムが返ってくる限り、printする。自分でイテレータを実装使用と思うと、これらの要素や定型句を理解する必要がる。一つずつ見ていこう。

## iter()

```rust
pub struct TreeIter<'a, T: 'a> {
    unvisited: Vec<&'a TreeNode<T>>
}

impl <'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}
```

```rust
impl<T> BinaryTree<T> {
    pub fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}
```

### 補足

ライフタイム

繰り返し実行する方法が複数あるなら、`iter`を用意せず、それぞれの方法に個別のを用意しましょう。

## IntoIterator

`IntoIterator`を実装している方を、イテレート可能と言う。

```rust
impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
```

### 補足

`impl Trait`

`iter`と`iter_mut`、一緒では？

## Iterator

```rust
impl <'a, T: 'a> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }

    // ベクタへの変換の実装をするなら、size_hint()を実装したほうがバッファの拡張のコストが下がる。
}

```

### 補足

`None`が返された後で、さらに`next()`を呼び出した場合の規定はない。絶対に`None`を呼びださえたい場合は、`fuse`アダプタなどを使いましょう。

## おまけ

### FromIterator

`FromIterator`は実装しなくても困らないが、イテレータでよく使う`.collect()`での理解にはちょうどよい。`.collect()`メソッドでは何もベクタにしかイテレータを集めることができないわけではない。

`from_iter()`を実装していれば、`.collect()`はそれを呼び出し、コレクションしてくれる。BinaryTreeに対して`.add`を一つずつ呼び出すのではなく、例えば、ベクタ`vec`があり、`BinaryTree::from_iter(vec.iter().cloned())`とすることでBinaryTreeがすぐ作れるようになる。

```rust
impl <T: Ord> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut tree = Empty;
        for x in iter {
            tree.add(x);
        }
        tree
    }
}
```

なお、アイテムを追加するたびにバッファサイズを追加する可能性があるようなコードは遅くなるので、必要なバッファサイズを事前に確保するように、先にアイテム数の最小値と最大値をわかっていればいい。そのためには、`size_hint()`メソッドを実装しておくと良い。(今回の`BinaryTree`の実装では、先に要素数の最小値はわかるが、最大値はわからない。)

### 遅延実行

アダプタ