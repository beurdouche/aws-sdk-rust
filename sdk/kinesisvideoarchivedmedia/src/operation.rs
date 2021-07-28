// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Downloads an MP4 file (clip) containing the archived, on-demand media from the
/// specified video stream over the specified time range. </p>
/// <p>Both the StreamName and the StreamARN parameters are optional, but you must specify
/// either the StreamName or the StreamARN when invoking this API operation. </p>
/// <p>As a prerequisite to using GetCLip API, you must obtain an endpoint using
/// <code>GetDataEndpoint</code>, specifying GET_CLIP for<code></code> the
/// <code>APIName</code> parameter. </p>
/// <p>An Amazon Kinesis video stream has the following requirements for providing data
/// through MP4:</p>
/// <ul>
/// <li>
/// <p>The media must contain h.264 or h.265 encoded video and, optionally, AAC or
/// G.711 encoded audio. Specifically, the codec ID of track 1 should be
/// <code>V_MPEG/ISO/AVC</code> (for h.264) or V_MPEGH/ISO/HEVC (for H.265).
/// Optionally, the codec ID of track 2 should be <code>A_AAC</code> (for AAC) or
/// A_MS/ACM (for G.711).</p>
/// </li>
/// <li>
/// <p>Data retention must be greater than 0.</p>
/// </li>
/// <li>
/// <p>The video track of each fragment must contain codec private data in the
/// Advanced Video Coding (AVC) for H.264 format and HEVC for H.265 format. For more
/// information, see <a href="https://www.iso.org/standard/55980.html">MPEG-4
/// specification ISO/IEC 14496-15</a>. For information about adapting
/// stream data to a given format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p>
/// </li>
/// <li>
/// <p>The audio track (if present) of each fragment must contain codec private data
/// in the AAC format (<a href="https://www.iso.org/standard/43345.html">AAC
/// specification ISO/IEC 13818-7</a>) or the <a href="http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html">MS
/// Wave format</a>.</p>
/// </li>
/// </ul>
/// <p>You can monitor the amount of outgoing data by monitoring the
/// <code>GetClip.OutgoingBytes</code> Amazon CloudWatch metric. For information about
/// using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video
/// Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS
/// Pricing</a>. Charges for outgoing AWS data apply.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetClip {
    _private: (),
}
impl GetClip {
    /// Creates a new builder-style object to manufacture [`GetClipInput`](crate::input::GetClipInput)
    pub fn builder() -> crate::input::get_clip_input::Builder {
        crate::input::get_clip_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse<smithy_http::body::SdkBody> for GetClip {
    type Output = std::result::Result<crate::output::GetClipOutput, crate::error::GetClipError>;
    fn parse_unloaded(
        &self,
        response: &mut http::Response<smithy_http::body::SdkBody>,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.status().is_success() && response.status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_get_clip(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_get_clip_error(response)
    }
}

/// <p>Retrieves an MPEG Dynamic Adaptive Streaming over HTTP (DASH) URL for the stream. You
/// can then open the URL in a media player to view the stream contents.</p>
/// <p>Both the <code>StreamName</code> and the <code>StreamARN</code> parameters are
/// optional, but you must specify either the <code>StreamName</code> or the
/// <code>StreamARN</code> when invoking this API operation.</p>
/// <p>An Amazon Kinesis video stream has the following requirements for providing data
/// through MPEG-DASH:</p>
/// <ul>
/// <li>
/// <p>The media must contain h.264 or h.265 encoded video and, optionally, AAC or
/// G.711 encoded audio. Specifically, the codec ID of track 1 should be
/// <code>V_MPEG/ISO/AVC</code> (for h.264) or V_MPEGH/ISO/HEVC (for H.265).
/// Optionally, the codec ID of track 2 should be <code>A_AAC</code> (for AAC) or
/// A_MS/ACM (for G.711).</p>
/// </li>
/// <li>
/// <p>Data retention must be greater than 0.</p>
/// </li>
/// <li>
/// <p>The video track of each fragment must contain codec private data in the
/// Advanced Video Coding (AVC) for H.264 format and HEVC for H.265 format. For more
/// information, see <a href="https://www.iso.org/standard/55980.html">MPEG-4
/// specification ISO/IEC 14496-15</a>. For information about adapting
/// stream data to a given format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p>
/// </li>
/// <li>
/// <p>The audio track (if present) of each fragment must contain codec private data
/// in the AAC format (<a href="https://www.iso.org/standard/43345.html">AAC
/// specification ISO/IEC 13818-7</a>) or the <a href="http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html">MS
/// Wave format</a>.</p>
/// </li>
/// </ul>
/// <p>The following procedure shows how to use MPEG-DASH with Kinesis Video Streams:</p>
/// <ol>
/// <li>
/// <p>Get an endpoint using <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_GetDataEndpoint.html">GetDataEndpoint</a>, specifying
/// <code>GET_DASH_STREAMING_SESSION_URL</code> for the <code>APIName</code>
/// parameter.</p>
/// </li>
/// <li>
/// <p>Retrieve the MPEG-DASH URL using <code>GetDASHStreamingSessionURL</code>.
/// Kinesis Video Streams creates an MPEG-DASH streaming session to be used for
/// accessing content in a stream using the MPEG-DASH protocol.
/// <code>GetDASHStreamingSessionURL</code> returns an authenticated URL (that
/// includes an encrypted session token) for the session's MPEG-DASH
/// <i>manifest</i> (the root resource needed for streaming with
/// MPEG-DASH).</p>
/// <note>
/// <p>Don't share or store this token where an unauthorized entity can access
/// it. The token provides access to the content of the stream. Safeguard the
/// token with the same measures that you use with your AWS credentials.</p>
/// </note>
/// <p>The media that is made available through the manifest consists only of the
/// requested stream, time range, and format. No other media data (such as frames
/// outside the requested window or alternate bitrates) is made available.</p>
/// </li>
/// <li>
/// <p>Provide the URL (containing the encrypted session token) for the MPEG-DASH
/// manifest to a media player that supports the MPEG-DASH protocol. Kinesis Video
/// Streams makes the initialization fragment and media fragments available through
/// the manifest URL. The initialization fragment contains the codec private data
/// for the stream, and other data needed to set up the video or audio decoder and
/// renderer. The media fragments contain encoded video frames or encoded audio
/// samples.</p>
/// </li>
/// <li>
/// <p>The media player receives the authenticated URL and requests stream metadata
/// and media data normally. When the media player requests data, it calls the
/// following actions:</p>
/// <ul>
/// <li>
/// <p>
/// <b>GetDASHManifest:</b> Retrieves an MPEG DASH
/// manifest, which contains the metadata for the media that you want to
/// playback.</p>
/// </li>
/// <li>
/// <p>
/// <b>GetMP4InitFragment:</b> Retrieves the MP4
/// initialization fragment. The media player typically loads the
/// initialization fragment before loading any media fragments. This
/// fragment contains the "<code>fytp</code>" and "<code>moov</code>" MP4
/// atoms, and the child atoms that are needed to initialize the media
/// player decoder.</p>
/// <p>The initialization fragment does not correspond to a fragment in a
/// Kinesis video stream. It contains only the codec private data for the
/// stream and respective track, which the media player needs to decode the
/// media frames.</p>
/// </li>
/// <li>
/// <p>
/// <b>GetMP4MediaFragment:</b> Retrieves MP4
/// media fragments. These fragments contain the "<code>moof</code>" and
/// "<code>mdat</code>" MP4 atoms and their child atoms, containing the
/// encoded fragment's media frames and their timestamps. </p>
/// <note>
/// <p>After the first media fragment is made available in a streaming
/// session, any fragments that don't contain the same codec private
/// data cause an error to be returned when those different media
/// fragments are loaded. Therefore, the codec private data should not
/// change between fragments in a session. This also means that the
/// session fails if the fragments in a stream change from having only
/// video to having both audio and video.</p>
/// </note>
/// <p>Data retrieved with this action is billable. See <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Pricing</a> for details.</p>
/// </li>
/// </ul>
/// </li>
/// </ol>
/// <note>
/// <p>For restrictions that apply to MPEG-DASH sessions, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p>
/// </note>
/// <p>You can monitor the amount of data that the media player consumes by monitoring the
/// <code>GetMP4MediaFragment.OutgoingBytes</code> Amazon CloudWatch metric. For
/// information about using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video
/// Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS
/// Pricing</a>. Charges for both HLS sessions and outgoing AWS data apply.</p>
/// <p>For more information about HLS, see <a href="https://developer.apple.com/streaming/">HTTP Live Streaming</a> on the
/// <a href="https://developer.apple.com">Apple Developer site</a>.</p>
/// <important>
/// <p>If an error is thrown after invoking a Kinesis Video Streams archived media API,
/// in addition to the HTTP status code and the response body, it includes the following
/// pieces of information: </p>
/// <ul>
/// <li>
/// <p>
/// <code>x-amz-ErrorType</code> HTTP header – contains a more specific error
/// type in addition to what the HTTP status code provides. </p>
/// </li>
/// <li>
/// <p>
/// <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to
/// AWS, the support team can better diagnose the problem if given the Request
/// Id.</p>
/// </li>
/// </ul>
/// <p>Both the HTTP status code and the ErrorType header can be utilized to make
/// programmatic decisions about whether errors are retry-able and under what
/// conditions, as well as provide information on what actions the client programmer
/// might need to take in order to successfully try again.</p>
/// <p>For more information, see the <b>Errors</b> section at
/// the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>.
/// </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDASHStreamingSessionURL {
    _private: (),
}
impl GetDASHStreamingSessionURL {
    /// Creates a new builder-style object to manufacture [`GetDashStreamingSessionUrlInput`](crate::input::GetDashStreamingSessionUrlInput)
    pub fn builder() -> crate::input::get_dash_streaming_session_url_input::Builder {
        crate::input::get_dash_streaming_session_url_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetDASHStreamingSessionURL {
    type Output = std::result::Result<
        crate::output::GetDashStreamingSessionUrlOutput,
        crate::error::GetDASHStreamingSessionURLError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_dash_streaming_session_url_error(response)
        } else {
            crate::operation_deser::parse_get_dash_streaming_session_url_response(response)
        }
    }
}

/// <p>Retrieves an HTTP Live Streaming (HLS) URL for the stream. You can then open the URL
/// in a browser or media player to view the stream contents.</p>
/// <p>Both the <code>StreamName</code> and the <code>StreamARN</code> parameters are
/// optional, but you must specify either the <code>StreamName</code> or the
/// <code>StreamARN</code> when invoking this API operation.</p>
/// <p>An Amazon Kinesis video stream has the following requirements for providing data
/// through HLS:</p>
/// <ul>
/// <li>
/// <p>The media must contain h.264 or h.265 encoded video and, optionally, AAC
/// encoded audio. Specifically, the codec ID of track 1 should be
/// <code>V_MPEG/ISO/AVC</code> (for h.264) or <code>V_MPEG/ISO/HEVC</code> (for
/// h.265). Optionally, the codec ID of track 2 should be <code>A_AAC</code>.</p>
/// </li>
/// <li>
/// <p>Data retention must be greater than 0.</p>
/// </li>
/// <li>
/// <p>The video track of each fragment must contain codec private data in the
/// Advanced Video Coding (AVC) for H.264 format or HEVC for H.265 format (<a href="https://www.iso.org/standard/55980.html">MPEG-4 specification ISO/IEC
/// 14496-15</a>). For information about adapting stream data to a given
/// format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p>
/// </li>
/// <li>
/// <p>The audio track (if present) of each fragment must contain codec private data
/// in the AAC format (<a href="https://www.iso.org/standard/43345.html">AAC
/// specification ISO/IEC 13818-7</a>).</p>
/// </li>
/// </ul>
/// <p>Kinesis Video Streams HLS sessions contain fragments in the fragmented MPEG-4 form
/// (also called fMP4 or CMAF) or the MPEG-2 form (also called TS chunks, which the HLS
/// specification also supports). For more information about HLS fragment types, see the
/// <a href="https://tools.ietf.org/html/draft-pantos-http-live-streaming-23">HLS
/// specification</a>.</p>
/// <p>The following procedure shows how to use HLS with Kinesis Video Streams:</p>
/// <ol>
/// <li>
/// <p>Get an endpoint using <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_GetDataEndpoint.html">GetDataEndpoint</a>, specifying
/// <code>GET_HLS_STREAMING_SESSION_URL</code> for the <code>APIName</code>
/// parameter.</p>
/// </li>
/// <li>
/// <p>Retrieve the HLS URL using <code>GetHLSStreamingSessionURL</code>. Kinesis
/// Video Streams creates an HLS streaming session to be used for accessing content
/// in a stream using the HLS protocol. <code>GetHLSStreamingSessionURL</code>
/// returns an authenticated URL (that includes an encrypted session token) for the
/// session's HLS <i>master playlist</i> (the root resource needed for
/// streaming with HLS).</p>
/// <note>
/// <p>Don't share or store this token where an unauthorized entity could access
/// it. The token provides access to the content of the stream. Safeguard the
/// token with the same measures that you would use with your AWS
/// credentials.</p>
/// </note>
/// <p>The media that is made available through the playlist consists only of the
/// requested stream, time range, and format. No other media data (such as frames
/// outside the requested window or alternate bitrates) is made available.</p>
/// </li>
/// <li>
/// <p>Provide the URL (containing the encrypted session token) for the HLS master
/// playlist to a media player that supports the HLS protocol. Kinesis Video Streams
/// makes the HLS media playlist, initialization fragment, and media fragments
/// available through the master playlist URL. The initialization fragment contains
/// the codec private data for the stream, and other data needed to set up the video
/// or audio decoder and renderer. The media fragments contain H.264-encoded video
/// frames or AAC-encoded audio samples.</p>
/// </li>
/// <li>
/// <p>The media player receives the authenticated URL and requests stream metadata
/// and media data normally. When the media player requests data, it calls the
/// following actions:</p>
/// <ul>
/// <li>
/// <p>
/// <b>GetHLSMasterPlaylist:</b> Retrieves an HLS
/// master playlist, which contains a URL for the
/// <code>GetHLSMediaPlaylist</code> action for each track, and
/// additional metadata for the media player, including estimated bitrate
/// and resolution.</p>
/// </li>
/// <li>
/// <p>
/// <b>GetHLSMediaPlaylist:</b> Retrieves an HLS
/// media playlist, which contains a URL to access the MP4 initialization
/// fragment with the <code>GetMP4InitFragment</code> action, and URLs to
/// access the MP4 media fragments with the <code>GetMP4MediaFragment</code>
/// actions. The HLS media playlist also contains metadata about the stream
/// that the player needs to play it, such as whether the
/// <code>PlaybackMode</code> is <code>LIVE</code> or
/// <code>ON_DEMAND</code>. The HLS media playlist is typically static
/// for sessions with a <code>PlaybackType</code> of <code>ON_DEMAND</code>.
/// The HLS media playlist is continually updated with new fragments for
/// sessions with a <code>PlaybackType</code> of <code>LIVE</code>. There is
/// a distinct HLS media playlist for the video track and the audio track
/// (if applicable) that contains MP4 media URLs for the specific track.
/// </p>
/// </li>
/// <li>
/// <p>
/// <b>GetMP4InitFragment:</b> Retrieves the MP4
/// initialization fragment. The media player typically loads the
/// initialization fragment before loading any media fragments. This
/// fragment contains the "<code>fytp</code>" and "<code>moov</code>" MP4
/// atoms, and the child atoms that are needed to initialize the media
/// player decoder.</p>
/// <p>The initialization fragment does not correspond to a fragment in a
/// Kinesis video stream. It contains only the codec private data for the
/// stream and respective track, which the media player needs to decode the
/// media frames.</p>
/// </li>
/// <li>
/// <p>
/// <b>GetMP4MediaFragment:</b> Retrieves MP4
/// media fragments. These fragments contain the "<code>moof</code>" and
/// "<code>mdat</code>" MP4 atoms and their child atoms, containing the
/// encoded fragment's media frames and their timestamps. </p>
/// <note>
/// <p>After the first media fragment is made available in a streaming
/// session, any fragments that don't contain the same codec private
/// data cause an error to be returned when those different media
/// fragments are loaded. Therefore, the codec private data should not
/// change between fragments in a session. This also means that the
/// session fails if the fragments in a stream change from having only
/// video to having both audio and video.</p>
/// </note>
/// <p>Data retrieved with this action is billable. See <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Pricing</a> for details.</p>
/// </li>
/// <li>
/// <p>
/// <b>GetTSFragment:</b> Retrieves MPEG TS
/// fragments containing both initialization and media data for all tracks
/// in the stream.</p>
/// <note>
/// <p>If the <code>ContainerFormat</code> is <code>MPEG_TS</code>, this
/// API is used instead of <code>GetMP4InitFragment</code> and
/// <code>GetMP4MediaFragment</code> to retrieve stream
/// media.</p>
/// </note>
/// <p>Data retrieved with this action is billable. For more information, see
/// <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Kinesis Video Streams pricing</a>.</p>
/// </li>
/// </ul>
/// </li>
/// </ol>
/// <p>A streaming session URL must not be shared between players. The service
/// might throttle a session if multiple media players are sharing it. For
/// connection limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p>
/// <p>You can monitor the amount of data that the media player consumes by monitoring the
/// <code>GetMP4MediaFragment.OutgoingBytes</code> Amazon CloudWatch metric. For
/// information about using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video
/// Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS
/// Pricing</a>. Charges for both HLS sessions and outgoing AWS data apply.</p>
/// <p>For more information about HLS, see <a href="https://developer.apple.com/streaming/">HTTP Live Streaming</a> on the
/// <a href="https://developer.apple.com">Apple Developer site</a>.</p>
/// <important>
/// <p>If an error is thrown after invoking a Kinesis Video Streams archived media API,
/// in addition to the HTTP status code and the response body, it includes the following
/// pieces of information: </p>
/// <ul>
/// <li>
/// <p>
/// <code>x-amz-ErrorType</code> HTTP header – contains a more specific error
/// type in addition to what the HTTP status code provides. </p>
/// </li>
/// <li>
/// <p>
/// <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to
/// AWS, the support team can better diagnose the problem if given the Request
/// Id.</p>
/// </li>
/// </ul>
/// <p>Both the HTTP status code and the ErrorType header can be utilized to make
/// programmatic decisions about whether errors are retry-able and under what
/// conditions, as well as provide information on what actions the client programmer
/// might need to take in order to successfully try again.</p>
/// <p>For more information, see the <b>Errors</b> section at
/// the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>.
/// </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetHLSStreamingSessionURL {
    _private: (),
}
impl GetHLSStreamingSessionURL {
    /// Creates a new builder-style object to manufacture [`GetHlsStreamingSessionUrlInput`](crate::input::GetHlsStreamingSessionUrlInput)
    pub fn builder() -> crate::input::get_hls_streaming_session_url_input::Builder {
        crate::input::get_hls_streaming_session_url_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetHLSStreamingSessionURL {
    type Output = std::result::Result<
        crate::output::GetHlsStreamingSessionUrlOutput,
        crate::error::GetHLSStreamingSessionURLError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_hls_streaming_session_url_error(response)
        } else {
            crate::operation_deser::parse_get_hls_streaming_session_url_response(response)
        }
    }
}

/// <p>Gets media for a list of fragments (specified by fragment number) from the archived
/// data in an Amazon Kinesis video stream.</p>
/// <note>
/// <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint.
/// Then send the <code>GetMediaForFragmentList</code> requests to this endpoint using
/// the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url
/// parameter</a>. </p>
/// </note>
/// <p>For limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p>
/// <important>
/// <p>If an error is thrown after invoking a Kinesis Video Streams archived media API,
/// in addition to the HTTP status code and the response body, it includes the following
/// pieces of information: </p>
/// <ul>
/// <li>
/// <p>
/// <code>x-amz-ErrorType</code> HTTP header – contains a more specific error
/// type in addition to what the HTTP status code provides. </p>
/// </li>
/// <li>
/// <p>
/// <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to
/// AWS, the support team can better diagnose the problem if given the Request
/// Id.</p>
/// </li>
/// </ul>
/// <p>Both the HTTP status code and the ErrorType header can be utilized to make
/// programmatic decisions about whether errors are retry-able and under what
/// conditions, as well as provide information on what actions the client programmer
/// might need to take in order to successfully try again.</p>
/// <p>For more information, see the <b>Errors</b> section at
/// the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>.
/// </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMediaForFragmentList {
    _private: (),
}
impl GetMediaForFragmentList {
    /// Creates a new builder-style object to manufacture [`GetMediaForFragmentListInput`](crate::input::GetMediaForFragmentListInput)
    pub fn builder() -> crate::input::get_media_for_fragment_list_input::Builder {
        crate::input::get_media_for_fragment_list_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse<smithy_http::body::SdkBody>
    for GetMediaForFragmentList
{
    type Output = std::result::Result<
        crate::output::GetMediaForFragmentListOutput,
        crate::error::GetMediaForFragmentListError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut http::Response<smithy_http::body::SdkBody>,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.status().is_success() && response.status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_get_media_for_fragment_list(
            response,
        ))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_get_media_for_fragment_list_error(response)
    }
}

/// <p>Returns a list of <a>Fragment</a> objects from the specified stream and
/// timestamp range within the archived data.</p>
/// <p>Listing fragments is eventually consistent. This means that even if the producer
/// receives an acknowledgment that a fragment is persisted, the result might not be
/// returned immediately from a request to <code>ListFragments</code>. However, results are
/// typically available in less than one second.</p>
/// <note>
/// <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint.
/// Then send the <code>ListFragments</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url
/// parameter</a>. </p>
/// </note>
/// <important>
/// <p>If an error is thrown after invoking a Kinesis Video Streams archived media API,
/// in addition to the HTTP status code and the response body, it includes the following
/// pieces of information: </p>
/// <ul>
/// <li>
/// <p>
/// <code>x-amz-ErrorType</code> HTTP header – contains a more specific error
/// type in addition to what the HTTP status code provides. </p>
/// </li>
/// <li>
/// <p>
/// <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to
/// AWS, the support team can better diagnose the problem if given the Request
/// Id.</p>
/// </li>
/// </ul>
/// <p>Both the HTTP status code and the ErrorType header can be utilized to make
/// programmatic decisions about whether errors are retry-able and under what
/// conditions, as well as provide information on what actions the client programmer
/// might need to take in order to successfully try again.</p>
/// <p>For more information, see the <b>Errors</b> section at
/// the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>.
/// </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFragments {
    _private: (),
}
impl ListFragments {
    /// Creates a new builder-style object to manufacture [`ListFragmentsInput`](crate::input::ListFragmentsInput)
    pub fn builder() -> crate::input::list_fragments_input::Builder {
        crate::input::list_fragments_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListFragments {
    type Output =
        std::result::Result<crate::output::ListFragmentsOutput, crate::error::ListFragmentsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fragments_error(response)
        } else {
            crate::operation_deser::parse_list_fragments_response(response)
        }
    }
}