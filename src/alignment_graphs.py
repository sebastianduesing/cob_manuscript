import csv
import os
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
    pass


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


if __name__ == "__main__":
    main()
