(function() {var implementors = {};
implementors["actix_ratelimit"] = [{"text":"impl&lt;T, S, B&gt; <a class=\"trait\" href=\"actix_service/transform/trait.Transform.html\" title=\"trait actix_service::transform::Transform\">Transform</a>&lt;S&gt; for <a class=\"struct\" href=\"actix_ratelimit/middleware/struct.RateLimiter.html\" title=\"struct actix_ratelimit::middleware::RateLimiter\">RateLimiter</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"actix/handler/trait.Handler.html\" title=\"trait actix::handler::Handler\">Handler</a>&lt;<a class=\"enum\" href=\"actix_ratelimit/enum.ActorMessage.html\" title=\"enum actix_ratelimit::ActorMessage\">ActorMessage</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"type\" href=\"actix/actor/trait.Actor.html#associatedtype.Context\" title=\"type actix::actor::Actor::Context\">Context</a>: <a class=\"trait\" href=\"actix/address/envelope/trait.ToEnvelope.html\" title=\"trait actix::address::envelope::ToEnvelope\">ToEnvelope</a>&lt;T, <a class=\"enum\" href=\"actix_ratelimit/enum.ActorMessage.html\" title=\"enum actix_ratelimit::ActorMessage\">ActorMessage</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"actix_service/trait.Service.html\" title=\"trait actix_service::Service\">Service</a>&lt;Request = <a class=\"struct\" href=\"actix_web/service/struct.ServiceRequest.html\" title=\"struct actix_web::service::ServiceRequest\">ServiceRequest</a>, Response = <a class=\"struct\" href=\"actix_web/service/struct.ServiceResponse.html\" title=\"struct actix_web::service::ServiceResponse\">ServiceResponse</a>&lt;B&gt;, Error = <a class=\"struct\" href=\"actix_http/error/struct.Error.html\" title=\"struct actix_http::error::Error\">AWError</a>&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"actix_service/trait.Service.html#associatedtype.Future\" title=\"type actix_service::Service::Future\">Future</a>: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: 'static,&nbsp;</span>","synthetic":false,"types":["actix_ratelimit::middleware::RateLimiter"]}];
implementors["actix_utils"] = [{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"actix_service/transform/trait.Transform.html\" title=\"trait actix_service::transform::Transform\">Transform</a>&lt;S&gt; for <a class=\"struct\" href=\"actix_utils/inflight/struct.InFlight.html\" title=\"struct actix_utils::inflight::InFlight\">InFlight</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"actix_service/trait.Service.html\" title=\"trait actix_service::Service\">Service</a>,&nbsp;</span>","synthetic":false,"types":["actix_utils::inflight::InFlight"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"actix_service/transform/trait.Transform.html\" title=\"trait actix_service::transform::Transform\">Transform</a>&lt;S&gt; for <a class=\"struct\" href=\"actix_utils/order/struct.InOrder.html\" title=\"struct actix_utils::order::InOrder\">InOrder</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"actix_service/trait.Service.html\" title=\"trait actix_service::Service\">Service</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"actix_service/trait.Service.html#associatedtype.Response\" title=\"type actix_service::Service::Response\">Response</a>: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"actix_service/trait.Service.html#associatedtype.Future\" title=\"type actix_service::Service::Future\">Future</a>: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"actix_service/trait.Service.html#associatedtype.Error\" title=\"type actix_service::Service::Error\">Error</a>: 'static,&nbsp;</span>","synthetic":false,"types":["actix_utils::order::InOrder"]},{"text":"impl&lt;S, E&gt; <a class=\"trait\" href=\"actix_service/transform/trait.Transform.html\" title=\"trait actix_service::transform::Transform\">Transform</a>&lt;S&gt; for <a class=\"struct\" href=\"actix_utils/timeout/struct.Timeout.html\" title=\"struct actix_utils::timeout::Timeout\">Timeout</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"actix_service/trait.Service.html\" title=\"trait actix_service::Service\">Service</a>,&nbsp;</span>","synthetic":false,"types":["actix_utils::timeout::Timeout"]}];
implementors["actix_web_httpauth"] = [{"text":"impl&lt;S, B, T, F, O&gt; <a class=\"trait\" href=\"actix_service/transform/trait.Transform.html\" title=\"trait actix_service::transform::Transform\">Transform</a>&lt;S&gt; for <a class=\"struct\" href=\"actix_web_httpauth/middleware/struct.HttpAuthentication.html\" title=\"struct actix_web_httpauth::middleware::HttpAuthentication\">HttpAuthentication</a>&lt;T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"actix_service/trait.Service.html\" title=\"trait actix_service::Service\">Service</a>&lt;Request = <a class=\"struct\" href=\"actix_web/service/struct.ServiceRequest.html\" title=\"struct actix_web::service::ServiceRequest\">ServiceRequest</a>, Response = <a class=\"struct\" href=\"actix_web/service/struct.ServiceResponse.html\" title=\"struct actix_web::service::ServiceResponse\">ServiceResponse</a>&lt;B&gt;, Error = <a class=\"struct\" href=\"actix_http/error/struct.Error.html\" title=\"struct actix_http::error::Error\">Error</a>&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"actix_service/trait.Service.html#associatedtype.Future\" title=\"type actix_service::Service::Future\">Future</a>: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"struct\" href=\"actix_web/service/struct.ServiceRequest.html\" title=\"struct actix_web::service::ServiceRequest\">ServiceRequest</a>, T) -&gt; O + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;O: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"actix_web/service/struct.ServiceRequest.html\" title=\"struct actix_web::service::ServiceRequest\">ServiceRequest</a>, <a class=\"struct\" href=\"actix_http/error/struct.Error.html\" title=\"struct actix_http::error::Error\">Error</a>&gt;&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"actix_web_httpauth/extractors/trait.AuthExtractor.html\" title=\"trait actix_web_httpauth::extractors::AuthExtractor\">AuthExtractor</a> + 'static,&nbsp;</span>","synthetic":false,"types":["actix_web_httpauth::middleware::HttpAuthentication"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()