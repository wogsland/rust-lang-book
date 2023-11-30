fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(", worldy world!");
        println!("{}", s);
    }
    //println!("{}", s); <-- s is out of scope here & thus unavailable

    {
        let s1 = String::from("George");
        let _s2 = s1;
        //println!("{}", s1); <-- error: "borrow of moved value"
    }

    {
        let mut s1 = String::from("George");
        let _s2 = s1.push_str(" of the Jungle");
        println!("{}", s1); // works
        //println!("{}", s2); // errors
    }

    {
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        println!("{}, {}!", s1, s2);
    }

    {
        let s1 = String::from("Hello");
        let (s2, length) = calculate_length(s1);
        println!("'{}' has length {}.", s2, length);
    }

    {
        let s1 = String::from("Also, hello.");
        let length = calculate_length_by_ref(&s1);
        println!("'{}' has length {}.", s1, length);
    }

    {
        let mut s = String::from("Dude");
        change(&mut s);
        println!("{}", s);
    }

    {
        let mut s = String::from("Dude");
        let _r1 = &mut s;
        //let _r2 = &mut s;  <-- no multiple borrows allowed
        //println!("{}, {}", _r1, _r2);
    }

    {
        let s = String::from("Hoi, Welt!");
        let hello = &s[..3];
        let world = &s[5..9];
        println!("{} + {}", hello, world);
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

fn change(g_string: &mut String) {
    g_string.push_str(", where's my car?");
}

// no dangling pointers
//fn dangle() -> &String { // dangle returns a reference to a String
//
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
