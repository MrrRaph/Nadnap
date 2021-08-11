use clap::{
    App, Arg, ArgGroup
};
use crate::constants::{args::*, groups::*};

pub fn add_scan_techniques(app: App<'static>) -> App<'static> {
    app
        .help_heading(scan_techniques::NAME)
        .arg(
            Arg::new(tcp_syn_scan::NAME)
                .long(tcp_syn_scan::LONG)
                .about(tcp_syn_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_connect_scan::NAME)
                .long(tcp_connect_scan::LONG)
                .about(tcp_connect_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_ack_scan::NAME)
                .long(tcp_ack_scan::LONG)
                .about(tcp_ack_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_window_scan::NAME)
                .long(tcp_window_scan::LONG)
                .about(tcp_window_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_maimon_scan::NAME)
                .long(tcp_maimon_scan::LONG)
                .about(tcp_maimon_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(udp_scan::NAME)
                .long(udp_scan::LONG)
                .about(udp_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_null_scan::NAME)
                .long(tcp_null_scan::LONG)
                .about(tcp_null_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_fin_scan::NAME)
                .long(tcp_fin_scan::LONG)
                .about(tcp_fin_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(tcp_xmas_scan::NAME)
                .long(tcp_xmas_scan::LONG)
                .about(tcp_xmas_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(scan_flags::NAME)
                .long(scan_flags::LONG)
                .about(scan_flags::HELP)
                .takes_value(true)
                .value_name(scan_flags::VALUE_NAME)
        )
        .arg(
            Arg::new(idle_scan::NAME)
                .long(idle_scan::LONG)
                .about(idle_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(sctp_init_scan::NAME)
                .long(sctp_init_scan::LONG)
                .about(sctp_init_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(sctp_cookie_echo_scan::NAME)
                .long(sctp_cookie_echo_scan::LONG)
                .about(sctp_cookie_echo_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(ip_protocol_scan::NAME)
                .long(ip_protocol_scan::LONG)
                .about(ip_protocol_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(ftp_bounce_scan::NAME)
                .short(ftp_bounce_scan::SHORT)
                .about(ftp_bounce_scan::HELP)
                .takes_value(false)
        )
        .group(
            ArgGroup::new(scan_techniques::NAME)
                .args(&[
                    tcp_syn_scan::NAME, tcp_ack_scan::NAME, tcp_connect_scan::NAME,
                    tcp_window_scan::NAME, tcp_maimon_scan::NAME, udp_scan::NAME,
                    tcp_null_scan::NAME, tcp_fin_scan::NAME, tcp_xmas_scan::NAME,
                    scan_flags::NAME, sctp_init_scan::NAME, sctp_cookie_echo_scan::NAME,
                    ip_protocol_scan::NAME, ftp_bounce_scan::NAME
                ])
                .multiple(true)
        )
}