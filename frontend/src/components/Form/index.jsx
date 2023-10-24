export function Form(props) {
  const {onSubmit: props$onSubmit, ...rest} = props;
  function handleSubmit(e) {
    e.preventDefault();
    if (props$onSubmit) return props$onSubmit.call(this, e);
  }
  return <form onSubmit={handleSubmit} {...rest} />;
}
