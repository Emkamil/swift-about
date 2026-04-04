project('lfbe-about', 'rust', version: '1.0.0-alpha')

i18n = import('i18n')
install_data(
    sources: ['data/licenses/gpl-3.0.txt', 'data/licenses/lgpl-3.0.txt', 'data/licenses/bsd-3-clause.txt'],
    install_dir: get_option('datadir') / 'lfbe/licenses'
)

i18n.gettext('lfbe-about', preset: 'glib')
