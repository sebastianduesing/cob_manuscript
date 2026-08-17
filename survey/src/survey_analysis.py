import csv
import os
import re
import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import wilcoxon

GUESS = "I would have to guess"
BFO = [
    "generically dependent continuant",
    "independent continuant",
    "occurrent",
    "specifically dependent continuant",
]
COB = [
    "anatomical entity",
    "characteristic",
    "information content entity",
    "material entity",
    "process",
]
ANCESTORS = sorted(BFO + COB)
QUESTION = r"^[\w\s]+'([-:\w\s\d]+)'[\w\s]+'([-:\w\s\d]+)'\?$"


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


def tidy_keys(survey_dict, untidy=False):
    """
    Abbreviate some fieldnames, or un-abbreviate them if untidy=True
    """
    output = {}
    tidy = {
        "Timestamp": "timestamp",
        "Which of these positions do you currently hold?": "position",
        "In which of these domains do you work? (If multiple, select the one in which you do the most work.)":
        "domain",
    }
    conversion_dict = tidy
    if untidy:
        untidy = {}
        for key, val in tidy.items():
            untidy[val] = key
        conversion_dict = untidy
    for index, rowdict in survey_dict.items():
        output[index] = {}
        for key, val in rowdict.items():
            if key in conversion_dict.keys():
                new_key = conversion_dict[key]
                output[index][new_key] = val
            else:
                output[index][key] = val
    return output


def ancestor(question):
    m = re.fullmatch(QUESTION, question)
    if m:
        ancestor = m.group(2)
    return ancestor


def tidy_vals(survey_dict):
    output = {}
    positions = [
        "Undergraduate student",
        "Graduate student",
        "Postdoctoral fellow",
        "Technician/specialist",
        "Assistant professor",
        "Associate professor",
        "Professor",
    ]
    domains = {
        "Biomedical science (experimentation, data generation, etc.)":
        "Biomedical science",
        "Data science (data analysis, data harmonization/integration, etc.)":
        "Data science",
        "Ontology development": "Ontology development",
    }
    for index, rowdict in survey_dict.items():
        output[index] = {}
        for key, val in rowdict.items():
            if key == "position":
                if val in positions:
                    output[index][key] = val
                else:
                    output[index][key] = "Other"
            elif key == "domain":
                if val in domains.keys():
                    output[index][key] = domains[val]
                else:
                    output[index][key] = "Other"
            else:
                output[index][key] = val
    return output


def make_answer_key(answer_key_tsv):
    output = {}
    ak_dict = tsv2dict(answer_key_tsv)
    for rowdict in ak_dict.values():
        output[rowdict["Question"]] = rowdict["Correct Answer"]
    return output


def increment(xdict, key):
    val = xdict[key]
    val += 1
    xdict[key] = val


def responses_by_participant(responses, ak):
    output = {}
    for index, rowdict in responses.items():
        overall = {
            "total": 0,
            "correct": 0,
            "guess": 0,
            "incorrect": 0,
        }
        by_ont = {}
        for ont in ["COB", "BFO"]:
            by_ont[ont] = {
                "total": 0,
                "correct": 0,
                "guess": 0,
                "incorrect": 0,
            }
        by_anc = {}
        for anc in ANCESTORS:
            by_anc[anc] = {
                "total": 0,
                "correct": 0,
                "guess": 0,
                "incorrect": 0,
            }

        for key, val in rowdict.items():
            if key in ak.keys():
                if val == GUESS:
                    r = "guess"
                elif val == ak[key]:
                    r = "correct"
                else:
                    r = "incorrect"
                increment(overall, "total")
                increment(overall, r)
                anc = ancestor(key)
                increment(by_anc[anc], "total")
                increment(by_anc[anc], r)
                if anc in BFO:
                    increment(by_ont["BFO"], "total")
                    increment(by_ont["BFO"], r)
                elif anc in COB:
                    increment(by_ont["COB"], "total")
                    increment(by_ont["COB"], r)

            output[index] = {
                "timestamp": rowdict["timestamp"],
                "position": rowdict["position"],
                "domain": rowdict["domain"],
                "overall performance": overall,
                "performance by ontology": by_ont,
                "performance by ancestor": by_anc
            }
    return output


def organize_for_participant_hist(data):
    output = {
        "all": {
            "subtitle": "All participants",
            "color": "xkcd:wine",
        },
        "Biomedical science": {
            "subtitle": "Biomedical scientists only",
            "color": "xkcd:bright blue",
        },
        "Data science": {
            "subtitle": "Data scientists only",
            "color": "xkcd:light red",
        },
        "Ontology development": {
            "subtitle": "Ontology developers only",
            "color": "xkcd:kelly green",
        },
    }
    for i in output.keys():
        output[i]["results"] = dict()
        for j in ["all", "COB", "BFO"]:
            output[i]["results"][j] = dict()
            for k in ["correct", "guess", "incorrect"]:
                output[i]["results"][j][k] = list()
        output[i]["results"]["all"]["total"] = 90
        output[i]["results"]["COB"]["total"] = 50
        output[i]["results"]["BFO"]["total"] = 40
    for rowdict in data.values():
        d = rowdict["domain"]
        for opt in rowdict["overall performance"].keys():
            if opt in ["correct", "guess", "incorrect"]:
                val = rowdict["overall performance"][opt]
                output["all"]["results"]["all"][opt].append(val)
                if d in output.keys():
                    output[d]["results"]["all"][opt].append(val)
        for ont in ["COB", "BFO"]:
            for opt in rowdict["performance by ontology"][ont].keys():
                if opt in ["correct", "guess", "incorrect"]:
                    val = rowdict["performance by ontology"][ont][opt]
                    output["all"]["results"][ont][opt].append(val)
                    if d in output.keys():
                        output[d]["results"][ont][opt].append(val)
    for pdict in output.values():
        pdict["n"] = len(pdict["results"]["all"]["correct"])
    return output


def participant_histogram(data, image_dir):
    q_counts = {"all": 90, "COB": 50, "BFO": 40}
    if not os.path.exists(image_dir):
        os.makedirs(image_dir)
    for qset in ["all", "COB", "BFO"]:
        if qset == "COB":
            qset_txt = "COB ancestor questions"
        elif qset == "BFO":
            qset_txt = "BFO ancestor questions"
        else:
            qset_txt = "all questions"
        for response in ["correct", "guess", "incorrect"]:
            resp_txt = "'I would have to guess'" if response == "guess" else response
            fname = os.path.join(
                                 image_dir,
                                 f"hist_participants_{qset}_{response}.png"
                             )

            q_count = q_counts[qset]
            bin = list(range(q_count + 1))
            bin_div = 5
            if qset == "all":
                bin_div = 10
            bin = bin[::bin_div]
            tick_labels = []
            for i in range(len(bin)):
                x = bin[i]
                y = bin[i+1]
                if bin[i] != bin[-2]:
                    tick_labels.append(f"[{x}, {y})")
                else:
                    tick_labels.append(f"[{x}, {y}]")
                    break

            fig, ((ax0, ax1), (ax2, ax3)) = plt.subplots(nrows=2, ncols=2)
            fig.subplots_adjust(wspace=0.15, hspace=0.30, left=0.1, bottom=0.1)
            title = f"Distribution of participants by # of {resp_txt} responses ({qset_txt})"
            fig.suptitle(title, size="xx-large")
            fig.set_size_inches(13, 10)
            fig.set_dpi(300)
            for (ax, group) in [
                (ax0, "all"),
                (ax1, "Biomedical science"),
                (ax2, "Data science"),
                (ax3, "Ontology development"),
            ]:
                dataset = np.array(data[group]["results"][qset][response])
                n = len(dataset)
                median = np.median(dataset)
                box_props = dict(boxstyle="round", facecolor="white")
                box_text = f"n = {n}\nmedian = {median}"
                ax.grid(True, axis="y")
                ax.set_axisbelow(True)
                ax.hist(
                         dataset,
                         bins=bin, linewidth=1,
                         edgecolor="white",
                         color=data[group]["color"],
                     )
                ax.set_title(data[group]["subtitle"])
                ax.set_xlabel(f"# of {resp_txt} responses (out of {q_count} questions)")
                ax.set_ylabel("# of participants")
                tick_start = bin_div / 2
                tick_end = (q_count + tick_start) - 1
                ax.set_xticks(
                              np.arange(tick_start, tick_end, bin_div),
                              labels=tick_labels,
                              size="x-small"
                          )
                if response == "correct":
                    position = 0.05
                else:
                    position = 0.70
                ax.text(
                        position,
                        0.95,
                        box_text,
                        transform=ax.transAxes,
                        verticalalignment="top",
                        bbox=box_props
                    )
            plt.savefig(fname, dpi=300)


def wsrt(data, dir):
    output_path = os.path.join(dir, "wilcoxon_srt_results.txt")
    file = open(output_path, "w")
    file.write("Results of Wilcoxon Signed Rank Test on the following:\n")
    banner = "="*60
    for group in data.keys():
        group_txt = data[group]["subtitle"]
        file.write(f"\n{banner}\n{group_txt}\n{banner}\n")
        for res in ["correct", "guess", "incorrect"]:
            bfo_scores = data[group]["results"]["BFO"][res]
            bfo_data = [i / 40 for i in bfo_scores]
            cob_scores = data[group]["results"]["COB"][res]
            cob_data = [i / 50 for i in cob_scores]
            stat, p_value = wilcoxon(bfo_data, cob_data)
            file.write(f"Participant {res} response rate, BFO vs COB questions:\n")
            file.write(f"\tStatistic: {stat}\n\tP-value: {p_value}\n")
    file.close()


def main():
    survey_dir = "survey"
    results_dir = os.path.join(survey_dir, "results")
    images_dir = os.path.join(results_dir, "images")
    responses_tsv = os.path.join(survey_dir, "survey_responses.tsv")
    answer_key_tsv = os.path.join(survey_dir, "answer_key.tsv")
    responses = tidy_vals(tidy_keys(tsv2dict(responses_tsv)))
    ak = make_answer_key(answer_key_tsv)
    by_participant = responses_by_participant(responses, ak)
    organized = organize_for_participant_hist(by_participant)
    participant_histogram(organized, images_dir)
    wsrt(organized, results_dir)


if __name__ == "__main__":
    main()
