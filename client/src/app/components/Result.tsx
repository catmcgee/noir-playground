import React from 'react'; 
import stripAnsi from 'strip-ansi';

type ResultProps = {
  result: any;
};

const Result: React.FC<ResultProps> = ({ result }) => {
  return (
    <div>
      {result 
        ? result.message 
          ? typeof result.message === 'object' 
            ? JSON.stringify(result.message, null, 2) 
            : stripAnsi(result.message)
          : typeof result === 'object' 
            ? JSON.stringify(result, null, 2) 
            : result
        : <p className="text-sm text-gray-500 mt-2">Click Test or Submit to see a result</p>}
    </div>
  );
};

export default Result; 
