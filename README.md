## rust-sm-slim

A simple http server compliant with [SageMaker Endpoint requirements](https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-inference-code.html).
The container will read the entire inference response and discard it. It will then sleep for a user specified amount 
of time, and finally respond with a payload of a user specified size. 

### Usage

The body sent to `/invocations` can be anything. The bytes are simply consumed and discarded. A user must send the
`X-SageMaker-Custom-Attributes` header and define in it (1) the amount of time the container should sleep before responding
to this particular request (2) the size of the response. The format is as follows:

```text
<sleep time millis>,<response size bytes>
```

Any request without the Custom Attributes header or with a header value not complying to the above format will have a 
`400 BadRequest` response returned.

The container also honors the `SAGEMAKER_BIND_TO_PORT` environment variable and will configure the server to listen
on the port specified by this variable. This allows the container to be used in a Multi-Model Endpoint.

#### Example

Ping:
```bash
$ curl -w "%{http_code}" http://127.0.0.1:8080/ping
200%
```

Invocation:
```bash
$ time curl -X POST -H 'X-SageMaker-Custom-Attribute: 500,12' http://127.0.0.1:8080/invocations
AAAAAAAAAAAA
curl -X POST -H 'X-SageMaker-Custom-Attribute: 500,12'   0.00s user 0.01s system 1% cpu 0.522 total
```

### Docker Image

To build, run:
```bash
docker build -t sm-slim .
```