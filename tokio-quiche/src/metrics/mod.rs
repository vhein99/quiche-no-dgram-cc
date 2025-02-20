// Copyright (C) 2025, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Metrics collected across QUIC connections.

pub mod labels;
pub mod tokio_task;

use foundations::telemetry::metrics::metrics;
use foundations::telemetry::metrics::Counter;
use foundations::telemetry::metrics::Gauge;
use foundations::telemetry::metrics::Histogram;
use foundations::telemetry::metrics::HistogramBuilder;
use foundations::telemetry::metrics::TimeHistogram;
use std::net::IpAddr;
use std::sync::Arc;

/// Trait to direct the metrics emitted by the crate to a Prometheus registry.
pub trait Metrics: Send + Sync + Clone + Unpin + 'static {
    /// Number of QUIC connections currently in memory
    fn connections_in_memory(&self) -> Gauge;

    /// Maximum number of writable QUIC streams in a connection
    fn maximum_writable_streams(&self) -> Histogram;

    /// Overhead of QUIC handshake processing stage
    fn handshake_time_seconds(
        &self, stage: labels::QuicHandshakeStage,
    ) -> TimeHistogram;

    /// Number of error and partial writes while sending QUIC packets
    fn write_errors(&self, reason: labels::QuicWriteError) -> Counter;

    /// Number of QUIC packets received where the CID could not be verified.
    fn invalid_cid_packet_count(&self, reason: crate::BoxError) -> Counter;

    /// Number of accepted QUIC Initial packets
    fn accepted_initial_packet_count(&self) -> Counter;

    /// Number of accepted QUIC Initial packets using expensive label(s)
    fn expensive_accepted_initial_packet_count(&self, peer_ip: IpAddr)
        -> Counter;

    /// Number of QUIC packets received but not associated with an active
    /// connection
    fn rejected_initial_packet_count(
        &self, reason: labels::QuicInvalidInitialPacketError,
    ) -> Counter;

    /// Number of QUIC packets received but not associated with an active
    /// connection using expensive label(s)
    fn expensive_rejected_initial_packet_count(
        &self, reason: labels::QuicInvalidInitialPacketError, peer_ip: IpAddr,
    ) -> Counter;

    /// Combined utilized bandwidth of all open connections (max over the past
    /// two minutes)
    fn utilized_bandwidth(&self) -> Gauge;

    /// The highest utilized bandwidh reported during the lifetime of the
    /// connection
    fn max_bandwidth_mbps(&self) -> Histogram;

    /// The highest momentary loss reported during the lifetime of the
    /// connection
    fn max_loss_pct(&self) -> Histogram;

    /// Number of UDP packets dropped when receiving
    fn udp_drop_count(&self) -> Counter;

    /// Number of failed quic handshakes
    fn failed_handshakes(&self, reason: labels::HandshakeError) -> Counter;

    /// Number of HTTP/3 connection closures generated locally
    fn local_h3_conn_close_error_count(&self, reason: labels::H3Error)
        -> Counter;

    /// Number of QUIC connection closures generated locally
    fn local_quic_conn_close_error_count(
        &self, reason: labels::QuicError,
    ) -> Counter;

    /// Number of HTTP/3 connection closures generated by peer
    fn peer_h3_conn_close_error_count(&self, reason: labels::H3Error) -> Counter;

    /// Number of QUIC connection closures generated by peer
    fn peer_quic_conn_close_error_count(
        &self, reason: labels::QuicError,
    ) -> Counter;

    // ==== tokio runtime metrics ====

    /// Histogram of task schedule delays
    fn tokio_runtime_task_schedule_delay_histogram(
        &self, task: &Arc<str>,
    ) -> TimeHistogram;

    /// Histogram of task poll durations
    fn tokio_runtime_task_poll_duration_histogram(
        &self, task: &Arc<str>,
    ) -> TimeHistogram;

    /// Helps us get a rough idea of if our waker is causing issues.
    fn tokio_runtime_task_total_poll_time_micros(
        &self, task: &Arc<str>,
    ) -> Counter;
}

/// Standard implementation of [`Metrics`] using
/// [`foundations::telemetry::metrics`].
#[derive(Default, Clone)]
pub struct DefaultMetrics;

impl Metrics for DefaultMetrics {
    fn connections_in_memory(&self) -> Gauge {
        quic::connections_in_memory()
    }

    fn maximum_writable_streams(&self) -> Histogram {
        quic::maximum_writable_streams()
    }

    fn handshake_time_seconds(
        &self, stage: labels::QuicHandshakeStage,
    ) -> TimeHistogram {
        quic::handshake_time_seconds(stage)
    }

    fn write_errors(&self, reason: labels::QuicWriteError) -> Counter {
        quic::write_errors(reason)
    }

    fn invalid_cid_packet_count(&self, reason: crate::BoxError) -> Counter {
        quic::invalid_cid_packet_count(reason.to_string())
    }

    fn accepted_initial_packet_count(&self) -> Counter {
        quic::accepted_initial_packet_count()
    }

    fn expensive_accepted_initial_packet_count(
        &self, peer_ip: IpAddr,
    ) -> Counter {
        quic::expensive_accepted_initial_packet_count(peer_ip)
    }

    fn rejected_initial_packet_count(
        &self, reason: labels::QuicInvalidInitialPacketError,
    ) -> Counter {
        quic::rejected_initial_packet_count(reason)
    }

    fn expensive_rejected_initial_packet_count(
        &self, reason: labels::QuicInvalidInitialPacketError, peer_ip: IpAddr,
    ) -> Counter {
        quic::expensive_rejected_initial_packet_count(reason, peer_ip)
    }

    fn utilized_bandwidth(&self) -> Gauge {
        quic::utilized_bandwidth()
    }

    fn max_bandwidth_mbps(&self) -> Histogram {
        quic::max_bandwidth_mbps()
    }

    fn max_loss_pct(&self) -> Histogram {
        quic::max_loss_pct()
    }

    fn udp_drop_count(&self) -> Counter {
        quic::udp_drop_count()
    }

    fn failed_handshakes(&self, reason: labels::HandshakeError) -> Counter {
        quic::failed_handshakes(reason)
    }

    fn local_h3_conn_close_error_count(
        &self, reason: labels::H3Error,
    ) -> Counter {
        quic::local_h3_conn_close_error_count(reason)
    }

    fn local_quic_conn_close_error_count(
        &self, reason: labels::QuicError,
    ) -> Counter {
        quic::local_quic_conn_close_error_count(reason)
    }

    fn peer_h3_conn_close_error_count(&self, reason: labels::H3Error) -> Counter {
        quic::peer_h3_conn_close_error_count(reason)
    }

    fn peer_quic_conn_close_error_count(
        &self, reason: labels::QuicError,
    ) -> Counter {
        quic::peer_quic_conn_close_error_count(reason)
    }

    // ==== tokio runtime metrics ====

    /// Histogram of task schedule delays
    fn tokio_runtime_task_schedule_delay_histogram(
        &self, task: &Arc<str>,
    ) -> TimeHistogram {
        tokio::runtime_task_schedule_delay_histogram(task)
    }

    /// Histogram of task poll durations
    fn tokio_runtime_task_poll_duration_histogram(
        &self, task: &Arc<str>,
    ) -> TimeHistogram {
        tokio::runtime_task_poll_duration_histogram(task)
    }

    /// Helps us get a rough idea of if our waker is causing issues.
    fn tokio_runtime_task_total_poll_time_micros(
        &self, task: &Arc<str>,
    ) -> Counter {
        tokio::runtime_task_total_poll_time_micros(task)
    }
}

#[metrics]
pub(crate) mod quic {
    /// Number of QUIC connections currently in memory
    pub fn connections_in_memory() -> Gauge;

    /// Maximum number of writable QUIC streams in a connection
    #[optional]
    #[ctor = HistogramBuilder { buckets: &[0.0, 5.0, 10.0, 100.0, 1000.0, 2000.0, 3000.0, 10000.0, 20000.0, 50000.0], }]
    pub fn maximum_writable_streams() -> Histogram;

    /// Overhead of QUIC handshake processing stage
    #[ctor = HistogramBuilder { buckets: &[1E-5, 2E-5, 5E-5, 1E-4, 2E-4, 5E-4, 1E-3, 2E-3, 5E-3, 1E-2, 2E-2, 5E-2, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0], }]
    pub fn handshake_time_seconds(
        stage: labels::QuicHandshakeStage,
    ) -> TimeHistogram;

    /// Number of error and partial writes while sending QUIC packets
    pub fn write_errors(reason: labels::QuicWriteError) -> Counter;

    /// Number of QUIC packets received where the CID could not be verified.
    pub fn invalid_cid_packet_count(reason: String) -> Counter;

    /// Number of accepted QUIC Initial packets
    pub fn accepted_initial_packet_count() -> Counter;

    /// Number of accepted QUIC Initial packets using expensive label(s)
    #[optional]
    pub fn expensive_accepted_initial_packet_count(peer_ip: IpAddr) -> Counter;

    /// Number of QUIC packets received but not associated with an active
    /// connection
    pub fn rejected_initial_packet_count(
        reason: labels::QuicInvalidInitialPacketError,
    ) -> Counter;

    /// Number of QUIC packets received but not associated with an active
    /// connection using expensive label(s)
    #[optional]
    pub fn expensive_rejected_initial_packet_count(
        reason: labels::QuicInvalidInitialPacketError, peer_ip: IpAddr,
    ) -> Counter;

    /// Combined utilized bandwidth of all open connections (max over the past
    /// two minutes)
    pub fn utilized_bandwidth() -> Gauge;

    /// The highest utilized bandwidh reported during the lifetime of the
    /// connection
    #[ctor = HistogramBuilder { buckets: &[0., 1., 2., 5., 10., 20., 50., 100., 200., 300., 500., 750., 1000., 1500., 2000., 2500., 3000., 3500., 4000., 4500., 5000., 6000., 7000., 10000.], }]
    pub fn max_bandwidth_mbps() -> Histogram;

    /// The highest momentary loss reported during the lifetime of the
    /// connection
    #[ctor = HistogramBuilder { buckets: &[0.0, 0.1, 0.2, 0.5, 1., 2., 3., 4., 5., 10., 15., 20., 25., 50., 100.], }]
    pub fn max_loss_pct() -> Histogram;

    /// Number of UDP packets dropped when receiving
    pub fn udp_drop_count() -> Counter;

    /// Number of failed quic handshakes
    pub fn failed_handshakes(reason: labels::HandshakeError) -> Counter;

    /// Number of HTTP/3 connection closures generated locally
    pub fn local_h3_conn_close_error_count(reason: labels::H3Error) -> Counter;

    /// Number of QUIC connection closures generated locally
    pub fn local_quic_conn_close_error_count(
        reason: labels::QuicError,
    ) -> Counter;

    /// Number of HTTP/3 connection closures generated by peer
    pub fn peer_h3_conn_close_error_count(reason: labels::H3Error) -> Counter;

    /// Number of QUIC connection closures generated by peer
    pub fn peer_quic_conn_close_error_count(reason: labels::QuicError)
        -> Counter;
}

#[metrics]
mod tokio {
    /// Histogram of task schedule delays
    #[ctor = HistogramBuilder { buckets: &[0.0, 1E-4, 2E-4, 3E-4, 4E-4, 5E-4, 6E-4, 7E-4, 8E-4, 9E-4, 1E-3, 1E-2, 2E-2, 4E-2, 8E-2, 1E-1, 1.0], }]
    pub fn runtime_task_schedule_delay_histogram(
        task: &Arc<str>,
    ) -> TimeHistogram;

    /// Histogram of task poll durations
    #[ctor = HistogramBuilder { buckets: &[0.0, 1E-4, 2E-4, 3E-4, 4E-4, 5E-4, 6E-4, 7E-4, 8E-4, 9E-4, 1E-3, 1E-2, 2E-2, 4E-2, 8E-2, 1E-1, 1.0], }]
    pub fn runtime_task_poll_duration_histogram(task: &Arc<str>)
        -> TimeHistogram;

    /// Helps us get a rough idea of if our waker is causing issues.
    pub fn runtime_task_total_poll_time_micros(task: &Arc<str>) -> Counter;
}

pub(crate) fn quic_expensive_metrics_ip_reduce(ip: IpAddr) -> Option<IpAddr> {
    const QUIC_INITIAL_METRICS_V4_PREFIX: u8 = 20;
    const QUIC_INITIAL_METRICS_V6_PREFIX: u8 = 32;

    let prefix = if ip.is_ipv4() {
        QUIC_INITIAL_METRICS_V4_PREFIX
    } else {
        QUIC_INITIAL_METRICS_V6_PREFIX
    };

    if let Ok(ip_net) = ipnetwork::IpNetwork::new(ip, prefix) {
        Some(ip_net.network())
    } else {
        None
    }
}
