use std::path::Path;

struct Installation {
    base: Path
}

impl Installation {
    fn new(base: &str) -> Installation {
        Installation {
            base: Path::new(&String::from(base))
        }
    }

    fn ruby_path(&self) -> &Path {
        self.base.join("app/server/native/osx/ruby/bin/ruby").as_path()
    }

    fn server_path(&self) -> &Path {
        self.base.join("app/server/bin/sonic-pi-server.rb").as_path()
    }

    fn exists(&self) -> bool {
        self.ruby_path().exists() && self.server_path().exists()
    }
}
