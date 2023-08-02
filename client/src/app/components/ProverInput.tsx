import React from "react";

type ProverInputProps = {
  proverInputs: Array<{ id: number; key: string; value: string }>;
  setProverInputs: (
    proverInputs: Array<{ id: number; key: string; value: string }>
  ) => void;
};

const ProverInput: React.FC<ProverInputProps> = ({
  proverInputs,
  setProverInputs,
}) => {
  let maxId = Math.max(...proverInputs.map((input) => input.id), 0);

  const handleAddProverInput = () => {
    maxId++;
    setProverInputs([...proverInputs, { id: maxId, key: "", value: "" }]);
  };

  const handleDeleteProverInput = (id: number) => {
    setProverInputs(proverInputs.filter((input) => input.id !== id));
  };

  return (
    <div className="mt-2">
      <div className="flex gap-2 mb-2">
        {" "}
        {/* Add key-value pair section */}
        <button
          onClick={handleAddProverInput}
          className="w-2/12 py-2 px-4 bg-blue-500 text-white rounded hover:bg-blue-600"
          style={{ backgroundColor: "#9f3fff" }}
        >
          Add
        </button>
      </div>

      <ul className="mt-4">
        {proverInputs.map(({ id, key, value }, idx) => (
          <li key={id} className="mb-2">
            <div className="flex justify-between">
              <input
                type="text"
                value={key}
                onChange={(e) =>
                  setProverInputs(
                    proverInputs.map((input) =>
                      input.id === id
                        ? { ...input, key: e.target.value }
                        : input
                    )
                  )
                }
                placeholder="Input key"
                className="w-5/12 p-2 rounded border border-gray-300 focus:border-blue-500 focus:outline-none"
              />
              <input
                type="text"
                value={value}
                onChange={(e) => {
                  setProverInputs(
                    proverInputs.map((input) =>
                      input.id === id
                        ? { ...input, value: e.target.value }
                        : input
                    )
                  );
                }}
                placeholder="Input value"
                className="w-5/12 p-2 rounded border border-gray-300 focus:border-blue-500 focus:outline-none"
              />

              <button
                onClick={() => handleDeleteProverInput(id)}
                className="w-2/12 text-center"
              >
                🗑️
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default ProverInput;
