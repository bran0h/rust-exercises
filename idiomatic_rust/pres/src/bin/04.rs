struct ClosedFile;
struct OpenFile;

struct File<State> {
    name: String,
    state: std::marker::PhantomData<State>,
}

impl File<ClosedFile> {
    fn open(self) -> File<OpenFile> {
        println!("Opening file: {}", self.name);
        File {
            name: self.name,
            state: std::marker::PhantomData,
        }
    }
}

impl File<OpenFile> {
    fn write(&self, data: &str) {
        println!("Writing: {}", data);
    }

    fn close(self) -> File<ClosedFile> {
        println!("Closing file: {}", self.name);
        File {
            name: self.name,
            state: std::marker::PhantomData,
        }
    }
}

fn main() {
    let file = File::<ClosedFile> {
        name: String::from("data.txt"),
        state: std::marker::PhantomData,
    };

    let file = file.open(); // Compile-time guarantee that file is open
    file.write("Hello, world!");
    let file = file.close(); // Now we can't write anymore!
}
