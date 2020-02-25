# VRME Design Blog: Networking 

## Abstract

Multiple components make up the Virtual Reality Meeting Environment (VRME). Each
of these components have different workflows which have varying networking
requirements, and sometimes rely on information from each other. This blog entry
aims to formulate a design for the networking architecture of the system.

Key subsystems:

1. Account management and authentication subsystem (AMA)
	+ Avatars
		2. Meeting session management subsystem (MSM)
	+ Presentation slides
		3. Voice chat subsystem (VC)
		4. View state relay subsystem (VSR)

## Networking Overview

The **Operating Systems Interconnection** (OSI) model has 7 layers of
abstraction, each of a different set of responsibilities.

```
DATA UNIT	LAYER
═════════	═════════════════════════
Data		[ Application Layer		]	┐
Data		[ Presentation Layer	]	├─ Software Layers
Data		[ Session Layer			]	┘
Segments	[ Transport Layer		]
Packets		[ Network Layer			]	┐
Frames		[ Data Link Layer		]	├─ Hardware Layers
Bits		[ Physical Layer		]	┘
```

We're mostly concerned with levels above and including the *Transport Layer*.

| Layer              | Relevant Standards and Protocols |
|--------------------|----------------------------------|
| Application Layer  | HTTP, SIP                        |
| Presentation Layer | MIME                             |
| Session Layer      | RTP, RTCP, H.245                 |
| Transport Layer    | TCP, UDP, SCTP, DCCP             |

### Transport Layer Protocols

#### Transmission Control Protocol (TCP)

TCP 3-way handshake:

```
Host A			---	SYN		-->		Host B
Gets SYN

Host A			<--	SYN-ACK	---		Host B
Gets SYN-ACK

Host A			---	ACK		-->		Host B
Gets ACK

[OK] Connection established
```

#### TCP vs UDP

| TCP                   | UDP                           |
|-----------------------|-------------------------------|
| Reliable              | Unreliable                    |
| Connection-orientated | Connectionless                |
| Segmented, windowing  | No windowing, no transmission |
| Sequenced             | No sequencing                 |
| Acknowledge segments  | No acknowledgements           |

This makes TCP suitable when reliability is important, while UDP is suitable
when losses can be acceptable in exchange for lower overhead and faster
transmission.

- The 3-way transmission model of TCP requires 3 times the networking latency
  between the two parties.

#### Secured Transmission

At the *Application Layer*:

- HTTPS (HTTP + TLS)
- TLS (TCP or other reliable transport protocol + authentication)

#### VoIP Protocols

- SIP:
	+ UDP-based
	+ Problems with NAT firewalls due to TCP/IP level address rewriting.
		* Addressable via STUN
- RTP:
	+ UDP-based

