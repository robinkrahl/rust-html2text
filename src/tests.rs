use super::render::text_renderer::TrivialDecorator;
use super::{from_read, from_read_with_decorator, TextDecorator};

/// Like assert_eq!(), but prints out the results normally as well
macro_rules! assert_eq_str {
    ($a:expr, $b:expr) => {
        if $a != $b {
            println!("<<<\n{}===\n{}>>>", $a, $b);
            assert_eq!($a, $b);
        }
    };
}
fn test_html(input: &[u8], expected: &str, width: usize) {
    assert_eq_str!(from_read(input, width), expected);
}

fn test_html_decorator<D>(input: &[u8], expected: &str, width: usize, decorator: D)
where
    D: TextDecorator,
{
    let output = from_read_with_decorator(input, width, decorator);
    assert_eq_str!(output, expected);
}

#[test]
fn test_table() {
    test_html(
        br##"
   <table>
     <tr>
       <td>1</td>
       <td>2</td>
       <td>3</td>
     </tr>
   </table>
"##,
        r#"───┬───┬────
1  │2  │3   
───┴───┴────
"#,
        12,
    );
}

#[test]
fn test_thead() {
    test_html(
        br##"
   <table>
     <thead>
       <tr>
         <th>Col1</th>
         <th>Col2</th>
         <th>Col3</th>
       </tr>
     </thead>
     <tbody>
       <tr>
         <td>1</td>
         <td>2</td>
         <td>3</td>
       </tr>
     </tbody>
   </table>
"##,
        r#"────┬────┬─────
Col1│Col2│Col3 
────┼────┼─────
1   │2   │3    
────┴────┴─────
"#,
        15,
    );
}

#[test]
fn test_colspan() {
    test_html(
        br##"
   <table>
     <tr>
       <td>1</td>
       <td>2</td>
       <td>3</td>
     </tr>
     <tr>
       <td colspan="2">12</td>
       <td>3</td>
     </tr>
     <tr>
       <td>1</td>
       <td colspan="2">23</td>
     </tr>
   </table>
"##,
        r#"───┬───┬────
1  │2  │3   
───┴───┼────
12     │3   
───┬───┴────
1  │23      
───┴────────
"#,
        12,
    );
}

#[test]
fn test_para() {
    assert_eq_str!(from_read(&b"<p>Hello</p>"[..], 10), "Hello\n");
}

#[test]
fn test_para2() {
    assert_eq_str!(
        from_read(&b"<p>Hello, world!</p>"[..], 20),
        "Hello, world!\n"
    );
}

#[test]
fn test_blockquote() {
    assert_eq_str!(
        from_read(
            &br#"<p>Hello</p>
    <blockquote>One, two, three</blockquote>
    <p>foo</p>
"#[..],
            12
        ),
        r#"Hello

> One, two,
> three

foo
"#
    );
}

#[test]
fn test_ul() {
    test_html(
        br#"
        <ul>
          <li>Item one</li>
          <li>Item two</li>
          <li>Item three</li>
        </ul>
     "#,
        r#"* Item one
* Item two
* Item
  three
"#,
        10,
    );
}

#[test]
fn test_ol1() {
    test_html(
        br#"
        <ol>
          <li>Item one</li>
          <li>Item two</li>
          <li>Item three</li>
        </ol>
     "#,
        r#"1. Item one
2. Item two
3. Item
   three
"#,
        11,
    );
}

#[test]
fn test_ol2() {
    test_html(
        br#"
        <ol>
          <li>Item one</li>
          <li>Item two</li>
          <li>Item three</li>
          <li>Item four</li>
          <li>Item five</li>
          <li>Item six</li>
          <li>Item seven</li>
          <li>Item eight</li>
          <li>Item nine</li>
          <li>Item ten</li>
        </ol>
     "#,
        r#"1.  Item one
2.  Item two
3.  Item three
4.  Item four
5.  Item five
6.  Item six
7.  Item seven
8.  Item eight
9.  Item nine
10. Item ten
"#,
        20,
    );
}

#[test]
fn test_ol_start() {
    test_html(
        br#"
        <ol start="3">
          <li>Item three</li>
          <li>Item four</li>
        </ol>
     "#,
        r#"3. Item three
4. Item four
"#,
        20,
    );
}

#[test]
fn test_ol_start_9() {
    test_html(
        br#"
        <ol start="9">
          <li>Item nine</li>
          <li>Item ten</li>
        </ol>
     "#,
        r#"9.  Item nine
10. Item ten
"#,
        20,
    );
}

#[test]
fn test_ol_start_neg() {
    test_html(
        br#"
        <ol start="-1">
          <li>Item minus one</li>
          <li>Item zero</li>
          <li>Item one</li>
        </ol>
     "#,
        r#"-1. Item minus one
0.  Item zero
1.  Item one
"#,
        20,
    );
}

#[test]
fn test_strip_nl() {
    test_html(
        br#"
        <p>
           One
           Two
           Three
        </p>
     "#,
        "One Two Three\n",
        40,
    );
}
#[test]
fn test_strip_nl2() {
    test_html(
        br#"
        <p>
           One
           <span>
               Two
           </span>
           Three
        </p>
     "#,
        "One Two Three\n",
        40,
    );
}
#[test]
fn test_strip_nl_tbl() {
    test_html(
        br#"
       <table>
         <tr>
            <td>
               One
               <span>
                   Two
               </span>
               Three
            </td>
          </tr>
        </table>
     "#,
        r"────────────────────
One Two Three       
────────────────────
",
        20,
    );
}
#[test]
fn test_unknown_element() {
    test_html(
        br#"
       <foo>
       <table>
         <tr>
            <td>
               One
               <span><yyy>
                   Two
               </yyy></span>
               Three
            </td>
          </tr>
        </table>
        </foo>
     "#,
        r"────────────────────
One Two Three       
────────────────────
",
        20,
    );
}
#[test]
fn test_strip_nl_tbl_p() {
    test_html(
        br#"
       <table>
         <tr>
            <td><p>
               One
               <span>
                   Two
               </span>
               Three
            </p></td>
          </tr>
        </table>
     "#,
        r"────────────────────
One Two Three       
────────────────────
",
        20,
    );
}
#[test]
fn test_pre() {
    test_html(
        br#"
       <pre>foo
bar
wib   asdf;
</pre>
<p>Hello</p>
     "#,
        r"foo
bar
wib   asdf;

Hello
",
        20,
    );
}
#[test]
fn test_link() {
    test_html(
        br#"
       <p>Hello, <a href="http://www.example.com/">world</a></p>"#,
        r"Hello, [world][1]

[1] http://www.example.com/
",
        80,
    );
}
#[test]
fn test_link2() {
    test_html(
        br#"
       <p>Hello, <a href="http://www.example.com/">world</a>!</p>"#,
        r"Hello, [world][1]!

[1] http://www.example.com/
",
        80,
    );
}

#[test]
fn test_link3() {
    test_html(
        br#"
       <p>Hello, <a href="http://www.example.com/">w</a>orld</p>"#,
        r"Hello, [w][1]orld

[1] http://www.example.com/
",
        80,
    );
}

#[test]
fn test_link_wrap() {
    test_html(
        br#"
       <a href="http://www.example.com/">Hello</a>"#,
        r"[Hello][1]

[1] http:/
/www.examp
le.com/
",
        10,
    );
}

#[test]
fn test_wrap() {
    test_html(
        br"<p>Hello, world.  Superlongwordreally</p>",
        r#"Hello,
world.
Superlon
gwordrea
lly
"#,
        8,
    );
}

#[test]
fn test_wrap2() {
    test_html(
        br"<p>Hello, world.  This is a long sentence with a
few words, which we want to be wrapped correctly.</p>",
        r#"Hello, world. This
is a long sentence
with a few words,
which we want to be
wrapped correctly.
"#,
        20,
    );
}

#[test]
fn test_wrap3() {
    test_html(
        br#"<p><a href="dest">http://example.org/blah/</a> one two three"#,
        r#"[http://example.org/blah/
][1] one two three

[1] dest
"#,
        25,
    );
}

#[test]
fn test_div() {
    test_html(
        br"<p>Hello</p><div>Div</div>",
        r#"Hello

Div
"#,
        20,
    );
    test_html(
        br"<p>Hello</p><div>Div</div><div>Div2</div>",
        r#"Hello

Div
Div2
"#,
        20,
    );
}

#[test]
fn test_img_alt() {
    test_html(
        br"<p>Hello <img src='foo.jpg' alt='world'></p>",
        "Hello [world]\n",
        80,
    );
}

#[test]
fn test_br() {
    test_html(br"<p>Hello<br/>World</p>", "Hello\nWorld\n", 20);
}

#[test]
fn test_br2() {
    test_html(br"<p>Hello<br/><br/>World</p>", "Hello\n\nWorld\n", 20);
}

#[test]
fn test_br3() {
    test_html(br"<p>Hello<br/> <br/>World</p>", "Hello\n\nWorld\n", 20);
}

#[test]
fn test_subblock() {
    test_html(
        br#"<div>
     <div>Here's a <a href="https://example.com/">link</a>.</div>
     <div><ul>
     <li>Bullet</li>
     <li>Bullet</li>
     <li>Bullet</li>
     </ul></div>
     </div>"#,
        r"Here's a [link][1].

* Bullet
* Bullet
* Bullet

[1] https://example.com/
",
        80,
    );
}

#[test]
fn test_controlchar() {
    test_html("Foo\u{0080}Bar".as_bytes(), "FooBar\n", 80);
    test_html("Foo\u{0080}Bar".as_bytes(), "FooB\nar\n", 4);
    test_html("FooBa\u{0080}r".as_bytes(), "FooB\nar\n", 4);
}

#[test]
fn test_nested_table_1() {
    test_html(
        br##"
   <table>
     <tr>
       <td>
          <table><tr><td>1</td><td>2</td><td>3</td></tr></table>
       </td>
       <td>
          <table><tr><td>4</td><td>5</td><td>6</td></tr></table>
       </td>
       <td>
          <table><tr><td>7</td><td>8</td><td>9</td></tr></table>
       </td>
     </tr>
     <tr>
       <td>
          <table><tr><td>1</td><td>2</td><td>3</td></tr></table>
       </td>
       <td>
          <table><tr><td>4</td><td>5</td><td>6</td></tr></table>
       </td>
       <td>
          <table><tr><td>7</td><td>8</td><td>9</td></tr></table>
       </td>
     </tr>
     <tr>
       <td>
          <table><tr><td>1</td><td>2</td><td>3</td></tr></table>
       </td>
       <td>
          <table><tr><td>4</td><td>5</td><td>6</td></tr></table>
       </td>
       <td>
          <table><tr><td>7</td><td>8</td><td>9</td></tr></table>
       </td>
     </tr>
   </table>
"##,
        r#"─┬─┬──┬─┬─┬──┬─┬─┬───
1│2│3 │4│5│6 │7│8│9  
─┼─┼──┼─┼─┼──┼─┼─┼───
1│2│3 │4│5│6 │7│8│9  
─┼─┼──┼─┼─┼──┼─┼─┼───
1│2│3 │4│5│6 │7│8│9  
─┴─┴──┴─┴─┴──┴─┴─┴───
"#,
        21,
    );
}

#[test]
fn test_nested_table_2() {
    test_html(
        br##"
   <table>
     <tr>
       <td>
          <table>
             <tr><td>1</td><td>a</td></tr>
             <tr><td>2</td><td>b</td></tr>
          </table>
       </td>
       <td><pre>one
two
three
four
five
</pre>
       </td>
     </tr>
   </table>
"##,
        r#"─┬───┬─────
1│a  │one  
─┼───│two  
2│b  │three
 │   │four 
 │   │five 
─┴───┴─────
"#,
        11,
    );
}

#[test]
fn test_h1() {
    test_html(
        br##"
   <h1>Hi</h1>
   <p>foo</p>
"##,
        r#"# Hi

foo
"#,
        21,
    );
}

#[test]
fn test_h3() {
    test_html(
        br##"
   <h3>Hi</h3>
   <p>foo</p>
"##,
        r#"### Hi

foo
"#,
        21,
    );
}

// General test that spacing is preserved
#[test]
fn test_pre2() {
    test_html(
        br##"<pre>Hello  sp
world</pre>"##,
        r#"Hello  sp
world
"#,
        21,
    );
}

// Check that spans work correctly inside <pre>
#[test]
fn test_pre_span() {
    test_html(
        br##"
<pre>Hello <span>$</span>sp
<span>Hi</span> <span>$</span><span>foo</span>
<span>Hi</span> <span>foo</span><span>, </span><span>bar</span>
</pre>"##,
        r#"Hello $sp
Hi $foo
Hi foo, bar
"#,
        21,
    );
}

// Check tab behaviour
#[test]
fn test_pre_tab() {
    test_html(b"<pre>\tworld</pre>", "        world\n", 40);
    test_html(b"<pre>H\tworld</pre>", "H       world\n", 40);
    test_html(b"<pre>He\tworld</pre>", "He      world\n", 40);
    test_html(b"<pre>Hel\tworld</pre>", "Hel     world\n", 40);
    test_html(b"<pre>Hell\tworld</pre>", "Hell    world\n", 40);
    test_html(b"<pre>Hello\tworld</pre>", "Hello   world\n", 40);
    test_html(b"<pre>Helloo\tworld</pre>", "Helloo  world\n", 40);
    test_html(b"<pre>Hellooo\tworld</pre>", "Hellooo world\n", 40);
    test_html(b"<pre>Helloooo\tworld</pre>", "Helloooo        world\n", 40);
}

#[test]
fn test_em_strong() {
    test_html(
        br##"
   <p>Hi <em>em</em> <strong>strong</strong></p>
"##,
        r#"Hi *em* **strong**
"#,
        21,
    );
}

#[test]
#[ignore] // Not yet fixed!
fn test_nbsp_indent() {
    test_html(
        br##"
   <div>Top</div>
   <div>&nbsp;Indented</div>
   <div>&nbsp;&nbsp;Indented again</div>
"##,
        r#"Top
Indented
Indented again
"#,
        21,
    );
}

#[test]
fn test_deeply_nested() {
    use ::std::iter::repeat;
    let html = repeat("<foo>").take(1000).collect::<Vec<_>>().concat();
    test_html(html.as_bytes(), "", 10);
}

#[test]
fn test_deeply_nested_table() {
    use ::std::iter::repeat;
    let html = repeat("<table><tr><td>hi</td><td>")
        .take(1000)
        .collect::<Vec<_>>()
        .concat()
        + &repeat("</td></tr></table>")
            .take(1000)
            .collect::<Vec<_>>()
            .concat();
    test_html(
        html.as_bytes(),
        r#"────┬─┬───
hi  │h│   
    │i│   
────┴─┴───
"#,
        10,
    );
}

#[test]
fn test_table_no_id() {
    let html = r#"<html><body><table>
        <tr>
            <td>hi, world</td>
        </tr>
    </table></body></html>"#;
    test_html(
        html.as_bytes(),
        r#"──────────
hi, world 
──────────
"#,
        10,
    );
}

#[test]
fn test_table_cell_id() {
    let html = r#"<html><body><table>
        <tr>
            <td id="bodyCell">hi, world</td>
        </tr>
    </table></body></html>"#;
    test_html(
        html.as_bytes(),
        r#"──────────
hi, world 
──────────
"#,
        10,
    );
}

#[test]
fn test_table_row_id() {
    let html = r#"<html><body><table>
        <tr id="bodyrow">
            <td>hi, world</td>
        </tr>
    </table></body></html>"#;
    test_html(
        html.as_bytes(),
        r#"──────────
hi, world 
──────────
"#,
        10,
    );
}

#[test]
fn test_table_table_id() {
    let html = r#"<html><body><table id="bodytable">
        <tr>
            <td>hi, world</td>
        </tr>
    </table></body></html>"#;
    test_html(
        html.as_bytes(),
        r#"──────────
hi, world 
──────────
"#,
        10,
    );
}

#[test]
fn test_header_width() {
    //0 size
    test_html(
        br##"
        <h2>
            <table>
                        <h3>Anything</h3>
            </table>
        </h2>
"##,
        r#"## ### A
## ### n
## ### y
## ### t
## ### h
## ### i
## ### n
## ### g
## 
## ────
"#,
        7,
    );
    //Underflow
    test_html(
        br##"
        <h2>
            <table>
                <h3>Anything</h3>
            </table>
        </h2>
"##,
        r#"## ### A
## ### n
## ### y
## ### t
## ### h
## ### i
## ### n
## ### g
## 
## ──
"#,
        5,
    );
}

#[test]
fn test_trivial_decorator() {
    test_html_decorator(
        br#"<div>
     <div>Here's a <a href="https://example.com/">link</a>.</div>
     <div><ul>
     <li>Bullet</li>
     <li>Bullet</li>
     <li>Bullet</li>
     </ul></div>
     </div>"#,
        r"Here's a link.

* Bullet
* Bullet
* Bullet
",
        80,
        TrivialDecorator::new(),
    );
}

#[test]
fn test_issue_16() {
    test_html(b"<ul><li><!----></li></ul>", "", 10);
}

#[test]
fn test_pre_br() {
    test_html(
        b"<pre>Foo<br>Bar</pre>",
        r#"Foo
Bar
"#,
        10,
    );
}

#[test]
fn test_pre_emptyline() {
    test_html(br#"<pre>X<span id="i"> </span></pre>"#, "X  \n", 10);
}

#[test]
fn test_link_id_longline() {
    test_html(
        br#"<a href="foo" id="i">quitelongline</a>"#,
        r#"[quitelong
line][1]

[1] foo
"#,
        10,
    );
}
