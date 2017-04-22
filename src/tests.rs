use {Error, LineNum, flatter, SFlattedText};


// ------------------------------------------------------------------
//  TEST

#[test]
fn add_first() {
    let flat = flatter("....").unwrap();
    assert!(flat == SFlattedText::from("....\n"));
}



#[test]
fn empty_input() {
    let flat = flatter("").unwrap();
    assert!(flat == SFlattedText::from(""));
}



#[test]
fn empty_lines() {
    let flat = flatter("

")
        .unwrap();
    assert!(flat == SFlattedText::from("\n\n"));

    let flat = flatter("
        ")
        .unwrap();
    assert!(flat == SFlattedText::from("\n\n"));

}


#[test]
fn some_lines_one_token() {
    let flat = flatter("....
....")
        .unwrap();
    assert!(flat == SFlattedText::from("....\n....\n"));


    let flat = flatter("....
....
....
....")
        .unwrap();
    assert!(flat == SFlattedText::from("....\n....\n....\n....\n"));
}



#[test]
fn some_tokens_root_level_empty_line_separator() {
    let flat = flatter("1111

2222

3333
")
        .unwrap();
    assert!(flat == SFlattedText::from("1111\n\n2222\n\n3333\n"));


    let flat = flatter("00
01
02

10
11
12
13

20

30

    ")
        .unwrap();
    assert!(flat == SFlattedText::from("00\n01\n02\n\n10\n11\n12\n13\n\n20\n\n30\n\n\n"));
}



#[test]
fn nested_indent() {
    let flat = flatter("
0
    01
    02
        020
        021
        023
            0230
            0231
")
        .unwrap();

    assert!(flat ==
            SFlattedText::from("
0
\u{2}01
02
\u{2}020
021
023
\u{2}0230
0231
\u{3}\u{3}\u{3}"));
}




#[test]
fn back_indent() {
    let flat = flatter("
0
    01
    02
        020
        021
        023
            0230
            0231
    03
    04
    05
1
")
        .unwrap();


    println!("{:?} _____________________", flat);
    assert!(flat ==
            SFlattedText::from("\n0\n\u{2}01\n02\n\u{2}020\n021\n023\n\u{2}0230\n0231\n\u{3}\u{3}03\n04\n05\n\u{3}1\n"));
}




#[test]
fn complex() {
    let flat = flatter("
0a
0b
    01a
    01b
        010.a
        010.b
        010.c
            010..a
            010..b
1a
1b
1c
    1.a
    1.b
    1.c
            1..a
            1..b
2a

3a
    30.a
    30.b

    31.a

    32.a
        32..a
            320a
            320b

            321a
            321b
4a
4b

5a
")
        .unwrap();

    println!("{:?} _____________________", flat);
    assert!(flat ==
            SFlattedText::from("\n0a\n0b\n\u{2}01a\n01b\n\u{2}010.a\n010.b\n010.c\n\u{2}010..\
                                a\n010..b\n\u{3}\u{3}\u{3}1a\n1b\n1c\n\u{2}1.a\n1.b\n1.\
                                c\n\u{2}1..a\n1..b\n\u{3}\u{3}2a\n\n3a\n\u{2}30.a\n30.b\n\n31.\
                                a\n\n32.a\n\u{2}32..\
                                a\n\u{2}320a\n320b\n\n321a\n321b\n\u{3}\u{3}\u{3}4a\n4b\n\n5a\n"));
}



#[test]
fn delimiters() {
    let flat = flatter("
0
     | 01a
     01b
     01c

     02a
     02b

        |020a
        |020b

        |  021a
        |021b
1a
1b
    11a
    |11b
    11c

    12a  |
    |12b  |
2a
    21a
    21b
    |
    |

")
        .unwrap();

    assert!(flat ==
            SFlattedText::from("\n0\n\u{2} 01a\n01b\n01c\n\n02a\n02b\n\n\u{2}020a\n020b\n\n  \
                                021a\n021b\n\u{3}\u{3}1a\n1b\n\u{2}11a\n11b\n11c\n\n12a  \n12b  \
                                \n\u{3}2a\n\u{2}21a\n21b\n\n\n\n\u{3}"));
}



#[test]
fn delimiters_start_end() {
    let flat = flatter("
0
     || 01a
     01b
     01c

     02a
     02b

        |020a
        ||020b

        |  021a
        |021b
1a
1b
    11a
    ||11b
    11c

    12a  ||
    |12b  ||
2a
    21a
    21b
    |
    |

")
        .unwrap();

    assert!(flat ==
            SFlattedText::from("
0
\u{2}| 01a
01b
01c

02a
02b

\u{2}020a
|020b

  021a
021b
\u{3}\u{3}1a
1b
\u{2}11a
|11b
11c

12a  |
12b  |
\u{3}2a
\u{2}21a
21b



\u{3}"));
}


#[test]
fn invalid_indentation() {
    let error = flatter("
aaa
    bbb
   ccc
")
        .unwrap_err();
    assert!(error ==
            Error {
        line: LineNum(4),
        desc: "invalid indentation".to_owned(),
    });


    let error = flatter("
aaa
    bbb
        cccc
            dddd
     eeee
    ffff
gggg
")
        .unwrap_err();

    assert!(error ==
            Error {
        line: LineNum(6),
        desc: "invalid indentation".to_owned(),
    });


    let error = flatter("
aaa
    bbb
        cccc
   |eeee
")
        .unwrap_err();

    assert!(error ==
            Error {
        line: LineNum(5),
        desc: "invalid indentation".to_owned(),
    });

}
