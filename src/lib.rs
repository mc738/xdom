
mod document;
mod elements;


pub fn doc_test() {
    let doc = document::Document::load(".xdom/test.json".to_string());


    println!("Title: {}", doc.title);

    println!("Document:");

    let dom = document::Dom::create(doc);

    // for child in doc.root.children {
    //     read_child(child, 1);
    // }

}


fn read_child(child: document::DocumentElement, tabCount: i8) {

    for i in 0..tabCount  {
        print!("\t");
    }

    println!("Type: {}\tValue: {}", child.elType, child.value);

    for child in child.children {
        read_child(child, tabCount + 1)
    }
}