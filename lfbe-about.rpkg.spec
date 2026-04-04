Name:           lfbe-about
Version:        1.0.1
Release:        1%{?dist}
Summary:        LFBE About Dialog
License:        GPL-3.0-or-later
URL:            https://github.com/Emkamil/lfbe-about

# Kluczowa zmiana: używamy makra Source0, które COPR/rpkg wypełni automatycznie
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext

%description
Modern about dialog for LFBE.

%prep
# Jeśli rpkg pakuje pliki bezpośrednio w archiwum (bez folderu nadrzędnego),
# używamy -c, aby rpmbuild sam stworzył katalog roboczy.
%setup -q -c -n %{name}-%{version}

%build
# Wchodzimy do katalogu źródłowego (jeśli -c stworzył dodatkowy poziom)
# rpkg często wypakowuje kod do podkatalogu o nazwie projektu
cd %{name} || cd .
cargo build --release --locked

%install
# Przejście do katalogu budowania (na wypadek specyficznej struktury rpkg)
cd %{name} || cd .

# 1. Instalacja binarki
install -D -m 0755 target/release/lfbe-about %{buildroot}%{_bindir}/lfbe-about

# 2. Instalacja licencji
mkdir -p %{buildroot}%{_datadir}/lfbe/licenses
# Zauważyłem na Pana screenie, że pliki mają końcówki .3.0.txt lub -3-clause.txt
# Używamy gwiazdki, aby złapać wszystkie
install -p -m 0644 data/licenses/*.txt %{buildroot}%{_datadir}/lfbe/licenses/

# 3. Instalacja tłumaczeń
mkdir -p %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES
install -p -m 0644 po/lfbe-about.mo %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES/lfbe-about.mo

%files
%{_bindir}/lfbe-about
%{_datadir}/lfbe/licenses/*.txt
%{_datadir}/locale/pl/LC_MESSAGES/lfbe-about.mo

%changelog
* Sat Apr 04 2026 Kamil - 1.0.1-1
- Absolute path fix for rpkg and directory traversal