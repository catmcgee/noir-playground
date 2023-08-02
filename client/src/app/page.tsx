"use client"; // This is a client component 👈🏽
import { useState, useEffect } from "react";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

import CodeEditor from "./components/CodeEditor";
import Instructions from "./components/Instructions";
import ProverInput from "./components/ProverInput";
import Verifier from "./components/Verifier";
import Result from "./components/Result";



export default function Home() {
  const [instructions, setInstructions] = useState("");
  const [code, setCode] = useState("fn main() {\n}");
  const [proverInputs, setProverInputs] = useState<Array<{ id: number, key: string, value: string }>>([]);
  const [verifierToml, setVerifierToml] = useState("");
  const [result, setResult] = useState("");
  const [challenge, setChallenge] = useState(1);
  const [isRunning, setIsRunning] = useState(false); 
  const [isOpen, setIsOpen] = useState(false);


  useEffect(() => {
    async function fetchData() {
      if (!challenge) return;
  
      const response = await fetch(`http://localhost:3000/challenges/${challenge}`);
      const data = await response.json();
      setInstructions(data.description);
    }

    fetchData();
  }, [challenge]);


  const handleChallengeSelect = (event: any) => {
    setChallenge(event.target.value);
  };

  const submitCode = async () => {
    setIsRunning(true); 
    setResult(""); 
    const proverInputsObject = proverInputs.reduce((accumulator: {[key: string]: any}, current) => {
        accumulator[current.key] = current.value;
        return accumulator;
    }, {});
    const data = {
      "code": code,
      "challenge_id": parseInt(challenge),
      "prover_inputs": proverInputsObject
    };
    console.log(data);

  console.log(data);
  const response = await fetch('http://localhost:3000/execute', {
    method: 'POST',
    mode: 'cors',
    headers: {
        'Content-Type': 'application/json',
        'Origin': 'http://localhost:9000' // Your client's address
    },
    body: JSON.stringify(
        data
    ),
});




  const result = await response.json();  // get the response data as json
  setIsRunning(false); 
  setResult(result);  // set result to state
};

const testCode = async () => {
  setIsRunning(true);
  setResult("");
  
  const proverInputsObject = proverInputs.reduce(
    (accumulator: { [key: string]: any }, current) => {
      accumulator[current.key] = current.value;
      return accumulator;
    },
    {}
  );
  
  const data = {
    code,
    challenge_id: parseInt(challenge),
    prover_inputs: proverInputsObject,
  };

  console.log(data);
  
  const response = await fetch(
    "http://localhost:3000/execute_test",
    {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        Origin: "http://localhost:9000", // Your client's address
      },
      body: JSON.stringify(data),
    }
  );

  const result = await response.json(); 
  setIsRunning(false); 
  setResult(result); 
};

const checkCode = async () => {
  setIsRunning(true);
  setResult("");

  const data = {
    code,
    challenge_id: parseInt(challenge),
    prover_inputs: {}, // No inputs needed for check
  };

  const response = await fetch(
    "http://localhost:3000/execute_check",
    {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        Origin: "http://localhost:9000", // Your client's address
      },
      body: JSON.stringify(data),
    }
  );

  const result = await response.json(); 
  setIsRunning(false); 
  setResult(result); 

  // Update the inputs if the request was successful
  if (response.ok) {
    const proverInputsArray = Object.entries(result)
    .map(([key, value], id) => ({ id, key, value: String(value) }));
    setProverInputs(proverInputsArray);
  }
};


return (
  <div className="h-screen w-full p-5 flex flex-col"style={{ backgroundColor: '#f6fafd' }}>
      <div className="p-4 bg-white rounded-xl shadow-md">
          <select value={challenge} onChange={handleChallengeSelect} className="p-2 w-full focus:outline-none">
              <option value="">Select challenge...</option>
              <option value="1">Challenge 1</option>
              <option value="2">Challenge 2</option>
              <option value="3">Challenge 3</option>
          </select>
      </div>

      <div className="flex flex-grow mt-5 overflow-auto">
          <div className="w-5/12 bg-white rounded-xl p-4 shadow-md">
              <Instructions instructions={instructions} />
          </div>

          <div className="w-6/12 ml-5 flex flex-col">
              <div className="bg-white rounded-xl flex-grow overflow-auto p-4 shadow-md">
                  <CodeEditor code={code} setCode={setCode} />
              </div>

             
              <div className="flex-grow overflow-y-scroll max-h-[400px] mt-4 bg-white p-4 rounded-xl shadow-md">
                  <Tabs>
                      <TabList>
                          <Tab>Prover Inputs</Tab>
                          <Tab>Result</Tab>
                      </TabList>
                      
                      <TabPanel>
  <ProverInput proverInputs={proverInputs} setProverInputs={setProverInputs} />
  {
    proverInputs.length === 0 &&
    <p className="text-sm text-gray-500 mt-2">Prover Inputs are the inputs supplied in Prover.toml, which the prover uses to send its witness values(both private and public).</p>
  }
</TabPanel>

                      <TabPanel>
                          <Result result={result} />
                      </TabPanel>
                  </Tabs>
              </div>

              {/* Define the buttons wrapper with flex class and shrink buttons to half size using w-1/2 */}
              <div className="flex mt-4">
              <div className="flex mt-4 justify-between"> {/* Added justify-between */}
              
              <button 
  onMouseEnter={() => setIsOpen(true)}
  onMouseLeave={() => setIsOpen(false)}
  onClick={checkCode} 
  className="w-1/3 py-2 px-4 bg-yellow-500 text-white rounded flex items-center justify-center relative">
  {isRunning ? 'Checking..' : 'Check'}
  <span className="ml-2">?</span>
  {isOpen && (
    <div 
      style={{
        position: 'absolute',
        bottom: '100%',
        left: '50%',
        transform: 'translateX(-50%)',
        color: 'black',
        background: 'white',
        padding: '10px',
        borderRadius: '8px',
        boxShadow: '0px 0px 1px rgba(0, 0, 0, 0.5)',
        zIndex: 1000,
        minWidth: '200px' 
      }}
    >
      Generate Prover.toml and fill in prover inputs
    </div>
  )}
</button>




  <button onClick={testCode} className="w-full py-2 px-4 bg-blue-500 text-white rounded mx-2" style={{ backgroundColor: '#9f3fff' }}>
    {isRunning ? 'Testing...' : 'Test'}
  </button>
  <button onClick={submitCode} className="w-full py-2 px-4 bg-green-500 text-white rounded ml-2" style={{ backgroundColor: '#4db324' }}>
    {isRunning ? "Submitting..." : "Submit"}
  </button>
</div>



              </div>
          </div>
      </div>
  </div>
);




};