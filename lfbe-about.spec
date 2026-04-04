Name:           lfbe-about
Version:        1.0.0
Release:        1%{?dist}
Summary:        About dialog for the LFBE Desktop Environment
License:        GPL-3.0-or-later
URL:            https://github.com/twoj-user/lfbe-about

Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext

%description
A modern and lightweight about dialog for the Lightweight Fast Beautiful Environment (LFBE),
built with Rust, GTK4 and Libadwaita.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release

%install
# Tworzenie struktury katalogów wewnątrz paczki
install -D -m 0755 target/release/lfbe-about %{buildroot}%{_bindir}/lfbe-about

# Jeśli masz plik .desktop lub ikony, dodaj je tutaj:
# install -D -m 0644 data/org.lfbe.about.desktop %{buildroot}%{_datadir}/applications/org.lfbe.about.desktop

%files
%{_bindir}/lfbe-about
# %{_datadir}/applications/org.lfbe.about.desktop

%changelog
* Sat Apr 04 2026 Kamil <kamil@B450-AORUS-PRO> - 1.0.0alpha-1
- Initial release of lfbe-about
