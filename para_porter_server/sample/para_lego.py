"""
このスクリプトは基準パラメータの値を変更した後に
そのほかの品目の過剰率を調べたいときに、手打ち入力しなければならない状態だったので
[円形検査設定1]～'[円形検査設定20]を指定して自動反映できるようにしたものです。

"""

from pathlib import Path


class ParaContent:
    def __init__(self,text_file_path:Path):
        self.text_file_path = text_file_path

        self.para_items:list = [
            '[品種設定データ]', '[画像情報]', '[明度検査]', '[寸法変換]', '[基準寸法]', 
            '[輪郭取得]', '[変形率検査]', '[明部ノイズ除去]', '[抽出画像出力]', '[バリ検出]',
            '[円形検査設定1]', '[円形検査設定2]', '[円形検査設定3]', '[円形検査設定4]', 
            '[円形検査設定5]', '[円形検査設定6]', '[円形検査設定7]', '[円形検査設定8]', 
            '[円形検査設定9]', '[円形検査設定10]', '[円形検査設定11]', '[円形検査設定12]', 
            '[円形検査設定13]', '[円形検査設定14]', '[円形検査設定15]', '[円形検査設定16]', 
            '[円形検査設定17]', '[円形検査設定18]', '[円形検査設定19]', '[円形検査設定20]', 
            '[サンプル寸法]', '[1.w1]', '[2.w2]', '[3.w3]', '[4.w4]', '[5.w5]', '[6.w6]', 
            '[7.予備]', '[8.予備]', '[9.予備]', '[10.予備]', '[11.予備]', '[12.予備]', 
            '[13.予備]', '[14.予備]', '[15.予備]', '[16.予備]', '[17.予備]', '[18.予備]', '[19.予備]', '[4.予備]'
        ]

        

        self.read_data = self._read_txt_file()

    def get_item_content(self,item_name:str) -> dict:
        """ 
            返すデータ形式
            {
                "item_name":"[円形検査設定1]",
                "content":"[円形検査設定1]"～"[円形検査設定2]"までの文字列
            }
            指定したパラメータファイルの項目の中身を取得する。
            contentの文字列の例([円形検査設定1]"～"[円形検査設定2])
            -----------------------------------
            検査ウィンドウ名=w1.横流れ-mh5b
            NG信号選択=0
            検査有無=0
            検査画像選択=0
            追従径選択=2
            外径補正値=1.05
            内径補正値=-1.05
            フィルタ１回目=13
            フィルタ２回目=2
            ずらし量=8
            ずらし閾値=5
            近傍=2
            極性=2
            結合前ノイズ値=30
            膨張収縮前ノイズ値=20
            膨張収縮選択0=0
            膨張収縮サイズX0=2
            膨張収縮サイズY0=1
            膨張収縮選択1=1
            膨張収縮サイズX1=4
            膨張収縮サイズY1=1
            膨張収縮選択2=0
            膨張収縮サイズX2=1
            膨張収縮サイズY2=1
            粒子結合量=10
            結合後ノイズ値=850
            取得X幅補正値=0.00
            取得Y幅補正値=0.00
            大不良X幅許容値=0.30
            大不良Y幅許容値=0.30
            小不良X幅許容値=0.08
            小不良Y幅許容値=0.20
            小不良個数許容値=1
            総面積許容値=50000
            X幅許容値補正値=0
            Y幅許容値補正値=0
            粒子寸法判定方法=0
            バリ補正有無=0
            バリ補正参照距離=1
            総面積許容値総面積許容値=50000
            -----------------------------------
        """

        start_item_number = self.para_items.index(item_name)
        next_item_name = self.para_items[start_item_number + 1]
        find_number1 = self.read_data.find(item_name)
        find_number2 = self.read_data.find(next_item_name)

        return {"item_name":item_name,"content":self.read_data[find_number1:find_number2]}

    def write_kensa_range(self,write_dict:dict)->dict:
        """
        検査する範囲は製品の太さによって変わるので計算する。
        現状は製品の全範囲のみ指定できるようにしている。
        "リング幅補正値=2.4\\nリング幅上限許容値="の間の数字を抜きだしてる
        """

        # "リング幅補正値=2.4\\nリング幅上限許容値="の間の数字を抜きだしてる
        hutosa_value = "リング幅基準値="
        hutosa_allow_value = "リング幅上限許容値="
        start_number = self.read_data.find(hutosa_value)
        end_number = self.read_data.find(hutosa_allow_value)
        hutosa = self.read_data[start_number:end_number]
        hutosa = float(hutosa.replace(hutosa_value,"").replace("\n",""))

        # 検査範囲は製品真ん中から開始して上下範囲を指定する。そのため製品の太さを2.4なら1.2にする。２分割する。
        # 製品の境界線をぎりぎりだと製品ではない空間まで範囲にしてしまい過剰になるため0.15狭くしている
        # それぞれ100をかけて、最終的に100で割っているのは丸め誤差対策。
        hutosa_range = ((((hutosa*100)) /2) -(0.15*100))/100

        # 外径補正値と内径補正値の内容を変更。
        # 「外径補正値」と「フィルタ1回目」の間を検索して内容変更。
        replace_text = "外径補正値=" + str(hutosa_range)+ "\n"+ "内径補正値=-" + str(hutosa_range)+ "\n"
        start_number = write_dict["content"].find("外径補正値")
        end_number = write_dict["content"].find("フィルタ１回目")
        search_text = write_dict["content"][start_number:end_number]
        write_dict["content"] = write_dict["content"].replace(search_text,replace_text)
        return write_dict

    def write_item_content(self,write_dict:dict) -> None:
        """
            テキスト内容を変更したい内容に変更する。
            まだファイル内容は書き換わらない。
        """
        if not type(write_dict) == dict:
            raise TypeError("write_dictの引数は辞書型の{'item':str,'content':str}の形でしか受け付けません。")

        start_item_number = self.para_items.index(write_dict["item_name"])
        next_item_name = self.para_items[start_item_number + 1]
        find_number1 = self.read_data.find(write_dict["item_name"])
        find_number2 = self.read_data.find(next_item_name)

        



        self.read_data = self.read_data.replace(self.read_data[find_number1:find_number2],write_dict["content"])

    def _read_txt_file(self) -> str :
        """
            テキストファイルを読み込む。インスタンスを生成した時点で実行される__init__で定義。
        """
        with open(self.text_file_path,encoding="shift-jis") as f:
            reader = f.read()
        return reader

    
    def write_file(self) -> None:
        """
            実行するとインスタンス内容をファイルとして書き換える。。
        """
        with open(self.text_file_path,"wt",encoding="shift-jis") as w:
            w.write(self.read_data)


def update_B_para_file():
    original_txt_path = Path("CO0013Z0(mh5b0-b).txt")
    # original_txt_path = Path("CO0013Q9(mh5a0-a).txt")

    copy_txt_files = [
        "CO0014C6(mh5b0-b).txt",
        "CO0017W2(mh5b0-b).txt",
        "CO6056A0(mh5b0-b).txt",
        "DO1484G0(mh5b0-b).txt",
        "mh5基準_B(mh5b0-b).txt"
    ]

    for text_file in copy_txt_files:
        copy_txt_path = Path(text_file)
        original_text_instance = ParaContent(original_txt_path)
        copy_text_instance = ParaContent(copy_txt_path)
        replace_item_datas =  [
            '[円形検査設定1]', '[円形検査設定2]', '[円形検査設定3]', '[円形検査設定4]', 
            '[円形検査設定5]', '[円形検査設定6]', '[円形検査設定7]', '[円形検査設定8]', 
            '[円形検査設定9]', '[円形検査設定10]', '[円形検査設定11]', '[円形検査設定12]', 
            '[円形検査設定13]', '[円形検査設定14]', '[円形検査設定15]', '[円形検査設定16]', 
            '[円形検査設定17]', '[円形検査設定18]', '[円形検査設定19]', '[円形検査設定20]', 
        ]
        
        for replace_data in replace_item_datas:
            original_item_dict = original_text_instance.get_item_content(replace_data)
            replace_data = copy_text_instance.write_kensa_range(original_item_dict)
            copy_text_instance.write_item_content(replace_data)
        
        copy_text_instance.write_file()
        print(f"{text_file}を{original_txt_path.name}のパラメータに変更しました")


def update_A_para_file():
    original_txt_path = Path("CO0013Q9(mh5a0-a).txt")
    # original_txt_path = Path("CO0013Z0(mh5b0-b).txt")

    copy_txt_files = [
        "DO1597G0(mh5a0-a).txt",
        "DO2147G0(mh5a0-a).txt",
        "mh5基準_A(mh5a0-a).txt",
    ]

    for text_file in copy_txt_files:
        copy_txt_path = Path(text_file)
        original_text_instance = ParaContent(original_txt_path)
        copy_text_instance = ParaContent(copy_txt_path)
        replace_item_datas =  [
            '[円形検査設定1]', '[円形検査設定2]', '[円形検査設定3]', '[円形検査設定4]', 
            '[円形検査設定5]', '[円形検査設定6]', '[円形検査設定7]', '[円形検査設定8]', 
            '[円形検査設定9]', '[円形検査設定10]', '[円形検査設定11]', '[円形検査設定12]', 
            '[円形検査設定13]', '[円形検査設定14]', '[円形検査設定15]', '[円形検査設定16]', 
            '[円形検査設定17]', '[円形検査設定18]', '[円形検査設定19]', '[円形検査設定20]', 
        ]
        
        for replace_data in replace_item_datas:
            original_item_dict = original_text_instance.get_item_content(replace_data)
            replace_data = copy_text_instance.write_kensa_range(original_item_dict)
            copy_text_instance.write_item_content(replace_data)
        
        copy_text_instance.write_file()
        print(f"{text_file}を{original_txt_path.name}のパラメータに変更しました")


def main():
  update_B_para_file()
  update_A_para_file()
    

if __name__ == '__main__':
    main()

