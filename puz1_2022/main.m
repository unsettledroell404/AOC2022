clear

%% import using MATLAB import tool
opts = delimitedTextImportOptions("NumVariables", 1);

% Specify range and delimiter
opts.DataLines = [1, Inf];
opts.Delimiter = ",";

% Specify column names and types
opts.VariableNames = "CalorieList";
opts.VariableTypes = "double";

% Specify file level properties
opts.ExtraColumnsRule = "ignore";
opts.EmptyLineRule = "read";%<-- also read the empty lines and replace with NaN

% Import the data
tbl = readtable("C:\Users\roelb\OneDrive - TU Eindhoven\Documents\Privé\adventofcode\puz12022\calorielist.txt", opts);

%% loop to make a list of total calories per elf (part one)
calList = tbl.CalorieList;
%start at elf 1
k = 1;
%make array of zeros equal to number of elfs
totCalPerElf = zeros(1,sum(isnan(calList))+1);
for i=1:length(calList)
   if ~isnan(calList(i))
       totCalPerElf(k) = totCalPerElf(k) + calList(i);
   else
       %read a NaN, move to the next elf
       k = k + 1;
   end
end

[M,I] = max(totCalPerElf);
disp(['Elf carrying highest number of calories is elf number ' num2str(I) '.'])
disp(['The highest number of calories carried by one elf is ' num2str(M) '.'])

%% part two: n calories top 3 elves

[~,ind] = sort(totCalPerElf);
%sum the last 3 elements
top3cal = sum(totCalPerElf(ind(end-2:end)));
disp(['The top 3 elves are carrying ' num2str(top3cal) ' calories together.'])




