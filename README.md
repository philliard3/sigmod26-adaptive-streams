A submission for SIGMOD 2026.

# Datasets

## ECG

We use a randomized order of items from the "dev" split of the ECG dataset from an open dataset, as determined by the RITA project.

https://github.com/ljmzlh/RITA

https://storage.googleapis.com/rita_resources/rita_dataset.tar.gz


Feifei Liu, Chengyu Liu, Lina Zhao, Xiangyu Zhang, Xiaoling Wu, Xiaoyan
Xu, Yulin Liu, Caiyun Ma, Shoushui Wei, Zhiqiang He, et al. 2018. An open
access database for evaluating the algorithms of electrocardiogram rhythm and
morphology abnormality detection. Journal of Medical Imaging and Health
Informatics 8, 7 (2018), 1368–1373.

The original version of the dataset is available under Creative Commons Attribution 4.0 International Public License,
from sources such as the original RITA repository linked, or

https://physionet.org/content/challenge-2020/1.0.2/training/cpsc_2018/#files-panel

## LLM - BoolQ

This dataset can be extracted largely as-is from the original source, but we have included our splits and shuffled versions.

https://github.com/google-research-datasets/boolean-questions

Per the dataset creators, BoolQ is released under the Creative Commons Share-Alike 3.0 license.

## Face - IMDB-Face

https://github.com/fwang91/IMDb-Face

This dataset is retreived using the author's script, producing a folder for "clean" faces, and within that a folder for each person's face images. Our program will read from that outer folder and traverse inwards.


# Running

