import csv
import os
import numpy as np
import matplotlib.pyplot as plt


def tsv2dict(path):
    """
    Return a dict from a TSV
    """
    with open(path, "r", encoding="UTF-8") as f:
        reader = csv.DictReader(f, delimiter="\t")
        output = {}
        index = 0
        for row in reader:
            output[index] = row
            index += 1
    return output


def dict2tsv(xdict, path):
    """
    Save a TSV from a dict
    """
    rows = sorted([i for i in xdict.keys()])
    fieldnames = [i for i in xdict[rows[0]].keys()]
    with open(path, "w", newline="\n", encoding="utf-8") as tsv:
        writer = csv.DictWriter(tsv, fieldnames=fieldnames, delimiter="\t")
        writer.writeheader()
        for row in rows:
            writer.writerow(xdict[row])


def draw_class_alignment_pie(classes, fname):
    aligned = 0
    unaligned = 0
    for rowdict in classes.values():
        if rowdict["In Namespace?"] != "True":
            continue
        if rowdict["Lowest COB Ancestor IRI"] == "":
            unaligned += 1
        else:
            aligned += 1
    data = [aligned, unaligned]
    labels = ["Has COB ancestor", "Has no COB ancestor"]
    title = "OBO classes with/without a COB ancestor (in-namespace classes only)"
    colors = ["xkcd:tangerine", "xkcd:azure"]
    fig, ax = plt.subplots()
    fig.suptitle(title, size="x-large")
    fig.set_size_inches(8, 5)
    fig.set_dpi(300)
    pie = ax.pie(data,
                 labels=labels,
                 colors=colors,
                 wedgeprops={"edgecolor": "white"})
    ax.pie_label(pie, "{absval:d}\n{frac:.1%}", textprops={"color": "black", "size": "large"})
    plt.savefig(fname, dpi=300)


def draw_ont_alignment_hist(data, fname):
    vals = []
    for index, rowdict in data.items():
        val = rowdict["Ratio of Aligned in-Namespace Classes to All in-Namespace Classes"]
        if val == "":
            continue
        val = float(val) * 100
        vals.append(val)
    fig, ax = plt.subplots()
    title = "Distribution of OBO Ontologies by # of In-Namespace Terms with a COB Ancestor"
    fig.suptitle(title, size="xx-large")
    fig.set_size_inches(13, 8)
    fig.set_dpi(300)
    dataset = np.array(vals)
    n = len(dataset)
    median = np.median(dataset).round(2)
    box_props = dict(boxstyle="round", facecolor="white")
    box_text = f"n = {n}\nmedian = {median}"
    bin = list(range(101))
    bin_div = 10
    bin = bin[::bin_div]
    tick_labels = []
    for i in range(len(bin)):
        x = bin[i]
        y = bin[i + 1]
        if bin[i] != bin[-2]:
            tick_labels.append(f"[{x}%, {y}%)")
        else:
            tick_labels.append(f"[{x}%, {y}%]")
            break
    ax.grid(True, axis="y")
    ax.set_axisbelow(True)
    ax.hist(
            dataset,
            bins=bin,
            linewidth=1,
            edgecolor="white",
            color="xkcd:wine"
        )
    ax.set_xlabel("Percentage of In-Namespace Terms with an Ancestor in COB", size="x-large")
    ax.set_ylabel("Number of Ontologies", size="x-large")
    tick_start = bin_div / 2
    tick_end = (100 + tick_start) - 1
    ax.set_xticks(
        np.arange(tick_start, tick_end, bin_div),
        labels=tick_labels,
        size="medium"
    )
    ax.text(
        0.70,
        0.95,
        box_text,
        transform=ax.transAxes,
        verticalalignment="top",
        bbox=box_props,
        size="large",
    )
    plt.savefig(fname, dpi=300)


def draw_root_hist(data, fname):
    vals = []
    for index, rowdict in data.items():
        val = rowdict["Unaligned Roots"]
        if val == "":
            continue
        val = int(val)
        vals.append(val)
    fig, ax = plt.subplots()
    title = "Distribution of OBO Ontologies by # of Unaligned Root Classes"
    fig.suptitle(title, size="xx-large")
    fig.set_size_inches(13, 8)
    fig.set_dpi(300)
    dataset = np.array(vals)
    n = len(dataset)
    median = np.median(dataset).round(2)
    box_props = dict(boxstyle="round", facecolor="white")
    box_text = f"n = {n}\nmedian = {median}"
    bin = [0, 1, 2, 4, 7, 11, 21, 51, 101, 100000000]
    tick_labels = ["0", "1", "[2, 3]", "[4, 6]", "[7, 10]", "[11, 20]", "[21-50]", "[51, 100]", ">100" ]
    hist, bin_edges = np.histogram(dataset, bin)
    ax.grid(True, axis="y")
    ax.set_axisbelow(True)
    ax.bar(range(len(hist)),
           hist,
           width=1,
           linewidth=1,
           edgecolor="white",
           color="xkcd:wine")
    ax.set_xlabel("Number of Unaligned Root Classes", size="x-large")
    ax.set_ylabel("Number of Ontologies", size="x-large")
    ax.set_xticks([i for i, j in enumerate(hist)], labels=tick_labels)
    ax.text(
        0.70,
        0.95,
        box_text,
        transform=ax.transAxes,
        verticalalignment="top",
        bbox=box_props,
        size="large",
    )
    plt.savefig(fname, dpi=300)


def main():
    results_dir = os.path.join("results")
    image_dir = os.path.join(results_dir, "images")
    if not os.path.exists(image_dir):
        os.makedirs(image_dir)
    class_tsv = os.path.join(results_dir, "obo_classes.tsv")
    analysis_tsv = os.path.join(results_dir, "alignment_analysis.tsv")
    if os.path.isfile(class_tsv):
        classes = tsv2dict(class_tsv)
        fname = os.path.join(image_dir, "class_alignment_pie.png")
        draw_class_alignment_pie(classes, fname)
    if os.path.isfile(analysis_tsv):
        analysis = tsv2dict(analysis_tsv)
        fname = os.path.join(image_dir, "ont_alignment_hist.png")
        draw_ont_alignment_hist(analysis, fname)
        fname = os.path.join(image_dir, "ont_root_hist.png")
        draw_root_hist(analysis, fname)


if __name__ == "__main__":
    main()
