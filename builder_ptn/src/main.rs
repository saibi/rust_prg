#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// 소프트웨어 패키지를 나타냅니다.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        // todo!("1")
        Dependency {
            name: self.name.clone(),
            version_expression: format!("^{}", self.version),
        }
    }
}

/// 패키지용 빌더입니다. `build()`를 사용하여 `Package` 자체를 만듭니다.
struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        // todo!("2")
        let package = Package {
            name: name.into(),
            version: String::new(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        };
        Self(package)
    }

    /// 패키지 버전을 설정합니다.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// 패키지 작성자를 설정합니다.
    fn authors(mut self, authors: Vec<String>) -> Self {
        // todo!("3")
        self.0.authors = authors;
        self
    }

    /// 종속 항목을 추가합니다.
    fn dependency(mut self, dependency: Dependency) -> Self {
        // todo!("4")
        self.0.dependencies.push(dependency);
        self
    }

    /// 언어를 설정합니다. 설정하지 않으면 언어가 기본적으로 None으로 설정됩니다.
    fn language(mut self, language: Language) -> Self {
        // todo!("5")
        self.0.language = Some(language);
        self
    }

    fn build(self) -> Package {
        self.0
    }
}

fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log = PackageBuilder::new("log")
        .version("0.4")
        .language(Language::Rust)
        .build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");
}
