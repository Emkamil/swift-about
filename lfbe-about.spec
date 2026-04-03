Name:           lfbe-about
Version:        1.0.0
Release:        1%{?dist}
Summary:        About dialog for LFBE Desktop Environment
License:        GPLv3
URL:            https://github.com/Emkamil/lfbe-about

BuildRequires:  rust-packaging
BuildRequires:  meson
BuildRequires:  gcc
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext

%description
About dialog for LFBE Desktop Environment.

%prep
%autosetup

%build
%meson
%meson_build

%install
%meson_install
# Tutaj Meson sam rozmieści pliki w /usr/share/lfbe/licenses/ oraz /usr/share/locale/

%files
%{_bindir}/lfbe-about
%{_datadir}/lfbe/licenses/*.txt
%{_datadir}/locale/*/LC_MESSAGES/*.mo

%changelog
* Sat Apr 04 2026 Kamil - 1.0.0-1
- Initial release with licenses and translations support.