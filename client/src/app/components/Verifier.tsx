import React from 'react'; // Make sure React is imported correctly

type VerifierProps = {
  toml: string;
};

const Verifier: React.FC<VerifierProps> = ({ toml }) => {
  return (
    <div>
      <h2>Verifier</h2>
      <p>{toml}</p>
    </div>
  );
};

export default Verifier; // Don't forget to export your component
