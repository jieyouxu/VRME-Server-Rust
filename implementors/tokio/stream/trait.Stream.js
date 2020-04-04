(function() {var implementors = {};
implementors["tokio"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"tokio/io/trait.AsyncBufRead.html\" title=\"trait tokio::io::AsyncBufRead\">AsyncBufRead</a>&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/io/struct.Lines.html\" title=\"struct tokio::io::Lines\">Lines</a>&lt;R&gt;","synthetic":false,"types":["tokio::io::util::lines::Lines"]},{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"tokio/io/trait.AsyncBufRead.html\" title=\"trait tokio::io::AsyncBufRead\">AsyncBufRead</a>&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/io/struct.Split.html\" title=\"struct tokio::io::Split\">Split</a>&lt;R&gt;","synthetic":false,"types":["tokio::io::util::split::Split"]},{"text":"impl <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/net/struct.TcpListener.html\" title=\"struct tokio::net::TcpListener\">TcpListener</a>","synthetic":false,"types":["tokio::net::tcp::listener::TcpListener"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/net/tcp/struct.Incoming.html\" title=\"struct tokio::net::tcp::Incoming\">Incoming</a>&lt;'_&gt;","synthetic":false,"types":["tokio::net::tcp::incoming::Incoming"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/net/unix/struct.Incoming.html\" title=\"struct tokio::net::unix::Incoming\">Incoming</a>&lt;'_&gt;","synthetic":false,"types":["tokio::net::unix::incoming::Incoming"]},{"text":"impl <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/net/struct.UnixListener.html\" title=\"struct tokio::net::UnixListener\">UnixListener</a>","synthetic":false,"types":["tokio::net::unix::listener::UnixListener"]},{"text":"impl <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/signal/unix/struct.Signal.html\" title=\"struct tokio::signal::unix::Signal\">Signal</a>","synthetic":false,"types":["tokio::signal::unix::Signal"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/stream/struct.Empty.html\" title=\"struct tokio::stream::Empty\">Empty</a>&lt;T&gt;","synthetic":false,"types":["tokio::stream::empty::Empty"]},{"text":"impl&lt;I&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/stream/struct.Iter.html\" title=\"struct tokio::stream::Iter\">Iter</a>&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,&nbsp;</span>","synthetic":false,"types":["tokio::stream::iter::Iter"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/stream/struct.Once.html\" title=\"struct tokio::stream::Once\">Once</a>&lt;T&gt;","synthetic":false,"types":["tokio::stream::once::Once"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/stream/struct.Pending.html\" title=\"struct tokio::stream::Pending\">Pending</a>&lt;T&gt;","synthetic":false,"types":["tokio::stream::pending::Pending"]},{"text":"impl&lt;K, V&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/stream/struct.StreamMap.html\" title=\"struct tokio::stream::StreamMap\">StreamMap</a>&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":false,"types":["tokio::stream::stream_map::StreamMap"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/sync/broadcast/struct.Receiver.html\" title=\"struct tokio::sync::broadcast::Receiver\">Receiver</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["tokio::sync::broadcast::Receiver"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/sync/mpsc/struct.Receiver.html\" title=\"struct tokio::sync::mpsc::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["tokio::sync::mpsc::bounded::Receiver"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/sync/mpsc/struct.UnboundedReceiver.html\" title=\"struct tokio::sync::mpsc::UnboundedReceiver\">UnboundedReceiver</a>&lt;T&gt;","synthetic":false,"types":["tokio::sync::mpsc::unbounded::UnboundedReceiver"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/sync/watch/struct.Receiver.html\" title=\"struct tokio::sync::watch::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["tokio::sync::watch::Receiver"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/time/delay_queue/struct.DelayQueue.html\" title=\"struct tokio::time::delay_queue::DelayQueue\">DelayQueue</a>&lt;T&gt;","synthetic":false,"types":["tokio::time::delay_queue::DelayQueue"]},{"text":"impl <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/time/struct.Interval.html\" title=\"struct tokio::time::Interval\">Interval</a>","synthetic":false,"types":["tokio::time::interval::Interval"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a>&gt; <a class=\"trait\" href=\"tokio/stream/trait.Stream.html\" title=\"trait tokio::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio/time/struct.Throttle.html\" title=\"struct tokio::time::Throttle\">Throttle</a>&lt;T&gt;","synthetic":false,"types":["tokio::time::throttle::Throttle"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()