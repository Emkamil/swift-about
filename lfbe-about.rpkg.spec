Name:           lfbe-about
Version:        1.0.1
Release:        1%{?dist}
Summary:        LFBE About Dialog
License:        GPL-3.0-or-later
URL:            https://github.com/Emkamil/lfbe-about

Source:         {{{ git_dir_pack }}}

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext

%description
Modern about dialog for LFBE.

%%prep
# rpkg wypakowuje źródła do folderu o nazwie projektu. 
# %setup -n %{name} sprawi, że rpmbuild wejdzie dokładnie tam, gdzie jest Cargo.toml
%setup -q -n %{name}

%build
# Po wejściu do %{name} przez %setup, jesteśmy w katalogu głównym projektu.
# Nie dodajemy żadnego 'cd', tylko od razu budujemy:
cargo build --release

%install
# 1. Instalacja binarki
install -D -m 0755 target/release/lfbe-about %{buildroot}%{_bindir}/lfbe-about

# 2. Instalacja licencji
mkdir -p %{buildroot}%{_datadir}/lfbe/licenses
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