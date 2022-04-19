(* ::Package:: *)

SetDirectory[ParentDirectory[NotebookDirectory[], 2]]


$libpath = FileNameJoin[{"target", "release", "libguess.dylib"}]


GuessNumber[min_, max_, times_] := LibraryFunctionLoad[$libpath, "guess_number_ffi", LinkObject, LinkObject][min, max, times]


min = 1;
max = GoldenRatio^24 // Round;
p = Log2[max] // N
times = 100000;

data = GuessNumber[min, max, times];
Histogram[
    <|"\:968f\:673a\:9009\:62e9" -> data[[All, 3]], "\:9ec4\:91d1\:5206\:5272" -> data[[All, 2]], "\:4e8c\:5206\:6cd5" -> data[[All, 1]]|>,
    Automatic,
    "Probability",
    ChartLegends -> Automatic,
    ChartStyle -> "Pastel"
]
PDF[BinomialDistribution[a, 1 / 2], n] /. a -> (2pp - 2)
FindDistributionParameters[data[[All, 2]], BinomialDistribution[a, 1 / GoldenRatio]]
FindDistributionParameters[data[[All, 3]], NegativeBinomialDistribution[a, 1 / 2]]



