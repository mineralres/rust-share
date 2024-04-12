/***********************************************************************
*	@company	上海全创信息科技有限公司
*	@file		demo_xmd_stock.cpp
*	@brief		xmdapi demo
*	@history	2022-8-26
*	@author		n-sight
*
*	Windows：	1.请确认包.h .cpp 以及 .lib 文件都在相同目录；或者VS项目配置属性中【附加包含目录】以及【附加库目录】和【附加项依赖】正确设置相关路径
*				2.预处理器定义 _CRT_SECURE_NO_WARNINGS ;
*				3.使用VS2013以上版本编译通过
*
*	Linux：		1.库文件和头文件在同一目录下时， g++ demo_xmd_stock.cpp -o demo -L. -lxfastmdapi
*				2.当库文件和源文件不在同一目录时，请注意相应路径的不同
				3.运行时若找不到动态库，可export $LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
***********************************************************************/

/*****注意：本demo只用于演示行情的处理，更多交易相关接口请参考交易相关demo*****/

#include "TORATstpXMdApi.h"
#include <stdio.h>
#include <stdlib.h>
#include <cstring>
#include <codecvt>
#include <vector>
#include <locale>
#include <iconv.h>

using namespace std;
using namespace TORALEV1API;

int code_convert(char *from_charset, char *to_charset, char *inbuf, size_t inlen, char *outbuf, size_t outlen)
{
	iconv_t cd;
	int rc;
	char **pin = &inbuf;
	char **pout = &outbuf;
	cd = iconv_open(to_charset, from_charset);
	if (cd == 0)
		return -1;
	memset(outbuf, 0, outlen);
	if (iconv(cd, pin, &inlen, pout, &outlen) == -1)
		return -1;
	iconv_close(cd);
	return 0;
}
std::string any2utf8(std::string in, std::string fromEncode, std::string toEncode)
{
	char *inbuf = (char *)in.c_str();
	int inlen = strlen(inbuf);
	int outlen = inlen * 3; // in case unicode 3 times than ascii
	char outbuf[outlen] = {0};
	int rst = code_convert((char *)fromEncode.c_str(), (char *)toEncode.c_str(), inbuf, inlen, outbuf, outlen);
	if (rst == 0)
	{
		return std::string(outbuf);
	}
	else
	{
		return in;
	}
}
std::string gbk2utf8(const char *in)
{
	return any2utf8(std::string(in), std::string("gbk"), std::string("utf-8"));
}

class DemoMdSpi : public CTORATstpXMdSpi
{
public:
	DemoMdSpi(CTORATstpXMdApi *api)
	{
		m_api = api;
	}

	~DemoMdSpi()
	{
	}

private:
	virtual void OnFrontConnected()
	{
		CTORATstpReqUserLoginField req_user_login_field;
		memset(&req_user_login_field, 0, sizeof(req_user_login_field));
		//登陆放在连接回调中以确保先连接后登录
		int ret = m_api->ReqUserLogin(&req_user_login_field, nRequestID++);
		if (ret != 0)
		{
			printf("ReqUserLogin fail, ret[%d]\n", ret);
		}
	}

	virtual void OnFrontDisconnected(int nReanson)
	{
		printf("MdApi OnFrontDisconnected\n");
	}

	virtual void OnRspUserLogin(CTORATstpRspUserLoginField *pRspUserLoginField,
				    CTORATstpRspInfoField *pRspInfoField, int nRequestID)
	{
		if (pRspInfoField->ErrorID == 0)
		{
			printf("login success!\n");

			//订阅行情
#if 0  //【订阅全市场】
			/***********************************************************************
			【订阅全市场】
			【上海】sub_arr中只有一个"00000000"的合约，且ExchangeID填TORA_TSTP_EXD_SSE
			【深圳】sub_arr中只有一个"00000000"的合约，且ExchangeID填TORA_TSTP_EXD_SZSE
			当sub_arr中只有一个"00000000"的合约且ExchangeID填0时，订阅全市场所有合约行情
			其它情况，订阅sub_arr集合中的合约行情

			【注意】
			只有股票代码和市场代码匹配时才能订阅到行情；港股通市场代码同深交所代码
			***********************************************************************/
			char* sub_arr[1];
			sub_arr[0] = (char*)"00000000";//全市场订阅
			char exchange_id = 0;
			int ret = m_api->SubscribeMarketData(sub_arr, sizeof(sub_arr) / sizeof(char*), exchange_id);
			if (ret != 0)

			{
				printf("SubscribeMarketData::fail, ret[%d]\n", ret);
			}
			else
				printf("SubscribeMarketData success, ret[%d]\n", ret);
#endif //【订阅全市场】

#if 1 //【订阅合约列表】
			/***********************************************************************
			【订阅合约列表】
			【上海】sub_arr中只有一个"00000000"的合约，且ExchangeID填TORA_TSTP_EXD_SSE
			【深圳】sub_arr中只有一个"00000000"的合约，且ExchangeID填TORA_TSTP_EXD_SZSE


			【注意】
			1.只有股票代码和市场代码匹配时才能订阅到行情；港股通市场代码同深交所代码
			2.单个市场
			***********************************************************************/
			char *sub_arr_list_sh[2]; //上海证券列表
			char *sub_arr_list_sz[2]; //深圳证券列表
			sub_arr_list_sh[0] = (char *)"10004485";
			sub_arr_list_sh[1] = (char *)"10004476";

			// sub_arr_list_sz[0] = (char *)"300750";
			// sub_arr_list_sz[1] = (char *)"123029";

			int ret_sh = m_api->SubscribeSPMarketData(sub_arr_list_sh, sizeof(sub_arr_list_sh) / sizeof(char *), TORA_TSTP_EXD_SSE);
			if (ret_sh != 0)
			{
				printf("SubscribeMarketData::fail, ret[%d]\n", ret_sh);
			}
			else
				printf("SubscribeMarketData success, ret[%d]\n", ret_sh);

				// int ret_sz = m_api->SubscribeMarketData(sub_arr_list_sz, sizeof(sub_arr_list_sz) / sizeof(char *), TORA_TSTP_EXD_SZSE);
				// if (ret_sz != 0)
				// {
				// 	printf("SubscribeMarketData::fail, ret[%d]\n", ret_sz);
				// }
				// else
				// 	printf("SubscribeMarketData success, ret[%d]\n", ret_sz);
#endif //【订阅合约列表】

#if 1 //订阅实时合成快照
      // char *sub_arr_rdd[1];
      // sub_arr_rdd[0] = (char *)"00000000"; //全市场订阅
      // int ret_rdd = m_api->SubscribeRapidMarketData(sub_arr_rdd, 1, '1');
      // if (ret_rdd != 0)
      // {
      // 	printf("SubscribeRapidMarketData fail, ret[%d]\n", ret_rdd);
      // }
      // else
      // 	printf("SubscribeRapidMarketData success, ret[%d]\n", ret_rdd);

#endif //订阅实时合成快照

#if 1 //查询行情，例如警示标志等 只有TCP行情源才支持该查询，组播方式不支持该查询
      // CTORATstpInquiryMarketDataField list_IMDF = {0, 0};

				// int ret3 = m_api->ReqInquiryMarketDataMirror(&list_IMDF, nRequestID++);
				// if (ret3 != 0)
				// {
				// 	printf("ReqInquiryMarketDataMirror[%d] fail! ret[%d] it's available through TCP ONLY!\n", nRequestID, ret3);
				// }
				// else
				// {
				// 	printf("ReqInquiryMarketDataMirror[%d]::OK! ret[%d]\n\n", nRequestID, ret3);
				// }

#endif //查询行情，例如警示标志等

		} // if (pRspInfoField->ErrorID == 0)
		else
		{
			printf("login fail, error_id[%d] error_msg[%s]\n",
			       pRspInfoField->ErrorID, pRspInfoField->ErrorMsg);
		}
	}

	/**************************以下为主要回调函数*****************************/

	///订阅行情应答
	virtual void OnRspSubMarketData(CTORATstpSpecificSecurityField *pSpecificSecurityField, CTORATstpRspInfoField *pRspInfoField)
	{
		printf("OnRspSubMarketData SecurityID[%s] ExchangeID[%c] Success!\n", pSpecificSecurityField->SecurityID, pSpecificSecurityField->ExchangeID);
	}
	///普通快照行情回调
	virtual void OnRtnMarketData(CTORATstpMarketDataField *pMarketDataField)
	{

		printf("OnRtnMarketData::security_id[%s] security_name[%s] ExchangeID[%c] upper_limit_price[%.2f] lower_limit_price[%.2f] last_price[% .2f] HighestPrice[% .2f] LowestPrice[% .2f] BidPrice1[% .2f] AskPrice1[% .2f] BidVolume1[%lld] AskVolume1[%lld] update_time[%s]\n",
		       pMarketDataField->SecurityID,
		       gbk2utf8(pMarketDataField->SecurityName).c_str(), //编码为GBK
		       pMarketDataField->ExchangeID,
		       pMarketDataField->UpperLimitPrice,
		       pMarketDataField->LowerLimitPrice,
		       pMarketDataField->LastPrice,
		       pMarketDataField->HighestPrice,
		       pMarketDataField->LowestPrice,
		       pMarketDataField->BidPrice1,
		       pMarketDataField->AskPrice1,
		       pMarketDataField->BidVolume1,
		       pMarketDataField->AskVolume1,
		       pMarketDataField->UpdateTime);
	}

	// ///普通快照行情回调
	virtual void OnRtnSPMarketData(CTORATstpMarketDataField *pMarketDataField)
	{

		printf("OnRtnSPMarketData::security_id[%s] security_name[%s] ExchangeID[%c] upper_limit_price[%.2f] lower_limit_price[%.2f] last_price[% .2f] HighestPrice[% .2f] LowestPrice[% .2f] BidPrice1[% .2f] AskPrice1[% .2f] BidVolume1[%lld] AskVolume1[%lld] update_time[%s]\n",
		       pMarketDataField->SecurityID,
		       gbk2utf8(pMarketDataField->SecurityName).c_str(), //编码为GBK
		       pMarketDataField->ExchangeID,
		       pMarketDataField->UpperLimitPrice,
		       pMarketDataField->LowerLimitPrice,
		       pMarketDataField->LastPrice,
		       pMarketDataField->HighestPrice,
		       pMarketDataField->LowestPrice,
		       pMarketDataField->BidPrice1,
		       pMarketDataField->AskPrice1,
		       pMarketDataField->BidVolume1,
		       pMarketDataField->AskVolume1,
		       pMarketDataField->UpdateTime);
	}

	///订阅合成快照应答
	virtual void OnRspSubRapidMarketData(CTORATstpSpecificSecurityField *pSpecificSecurityField, CTORATstpRspInfoField *pRspInfoField)
	{
		printf("OnRspSubRapidMarketData SecurityID[%s] ExchangeID[%c] Success!\n", pSpecificSecurityField->SecurityID, pSpecificSecurityField->ExchangeID);
	};

	virtual void OnRspSubSPMarketData(CTORATstpSpecificSecurityField *pSpecificSecurityField, CTORATstpRspInfoField *pRspInfoField)
	{
		printf("OnRspSubSPMarketData SecurityID[%s] ExchangeID[%c] Success!\n", pSpecificSecurityField->SecurityID, pSpecificSecurityField->ExchangeID);
	}

	//风险警示标志HWLevel查询
	virtual void OnRspInquiryMarketDataMirror(CTORATstpMarketDataField *pMarketDataField, CTORATstpRspInfoField *pRspInfoField, int nRequestID, bool bIsLast)
	{
		if (bIsLast)
		{
			printf("OnRspInquiryMarketDataMirror::最后一行空记录 查询结束.\n");
		}
		else if (pMarketDataField) //只过滤有警示标志的记录
		{
			printf("OnRspInquiryMarketDataMirror::security_id[%s] security_name[%s] ExchangeID[%c] UpperLimitPrice[%.2f] LowerLimitPrice[%.2f] LastPrice[%.2f]\n",
			       pMarketDataField->SecurityID,
			       gbk2utf8(pMarketDataField->SecurityName).c_str(),
			       pMarketDataField->ExchangeID,
			       pMarketDataField->UpperLimitPrice,
			       pMarketDataField->LowerLimitPrice,
			       pMarketDataField->LastPrice);
		}
		else if (!pMarketDataField)
		{
			printf("OnRspInquiryMarketDataMirror::查询错误! nRequestID[%d]--ErrorID[%d]--ErrorMsg[%s]\n", nRequestID, pRspInfoField->ErrorID, pRspInfoField->ErrorMsg);
		}
	}

	///深度行情通知，实时合成快照行情
	virtual void OnRtnRapidMarketData(CTORATstpRapidMarketDataField *pRapidMarketData)
	{
		printf("%s,%c,%d,%lld,%lld,%lf,%lf,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lld,%lf,%lf,%lf,%lf,%lf,%lf,%lf,%c,%f,%lld,%lld\n",
		       pRapidMarketData->SecurityID,
		       pRapidMarketData->ExchangeID,
		       pRapidMarketData->DataTimeStamp,
		       pRapidMarketData->NumTrades,
		       pRapidMarketData->TotalVolumeTrade,
		       pRapidMarketData->TotalValueTrade,
		       pRapidMarketData->LastPrice,
		       pRapidMarketData->BidPrice1,
		       pRapidMarketData->BidVolume1,
		       pRapidMarketData->AskPrice1,
		       pRapidMarketData->AskVolume1,
		       pRapidMarketData->AskPrice2,
		       pRapidMarketData->AskVolume2,
		       pRapidMarketData->AskPrice3,
		       pRapidMarketData->AskVolume3,
		       pRapidMarketData->BidPrice2,
		       pRapidMarketData->BidVolume2,
		       pRapidMarketData->BidPrice3,
		       pRapidMarketData->BidVolume3,
		       pRapidMarketData->AskPrice4,
		       pRapidMarketData->AskVolume4,
		       pRapidMarketData->AskPrice5,
		       pRapidMarketData->AskVolume5,
		       pRapidMarketData->BidPrice4,
		       pRapidMarketData->BidVolume4,
		       pRapidMarketData->BidPrice5,
		       pRapidMarketData->BidVolume5,
		       pRapidMarketData->AskPrice6,
		       pRapidMarketData->AskVolume6,
		       pRapidMarketData->AskPrice7,
		       pRapidMarketData->AskVolume7,
		       pRapidMarketData->BidPrice6,
		       pRapidMarketData->BidVolume6,
		       pRapidMarketData->BidPrice7,
		       pRapidMarketData->BidVolume7,
		       pRapidMarketData->AskPrice8,
		       pRapidMarketData->AskVolume8,
		       pRapidMarketData->AskPrice9,
		       pRapidMarketData->AskVolume9,
		       pRapidMarketData->BidPrice8,
		       pRapidMarketData->BidVolume8,
		       pRapidMarketData->BidPrice9,
		       pRapidMarketData->BidVolume9,
		       pRapidMarketData->BidPrice10,
		       pRapidMarketData->BidVolume10,
		       pRapidMarketData->AskPrice10,
		       pRapidMarketData->AskVolume10,
		       pRapidMarketData->UpperLimitPrice,
		       pRapidMarketData->LowerLimitPrice,
		       pRapidMarketData->PreClosePrice,
		       pRapidMarketData->OpenPrice,
		       pRapidMarketData->HighestPrice,
		       pRapidMarketData->LowestPrice,
		       pRapidMarketData->ClosePrice,
		       pRapidMarketData->MDSecurityStat,
		       pRapidMarketData->IOPV,
		       pRapidMarketData->InnerSell,
		       pRapidMarketData->OuterBuy);
	}

private:
	CTORATstpXMdApi *m_api;
	int nRequestID = 0;
};

int main(int argc, char *argv[])
{
	/*********************************************demo运行说明************************************
	 * argv[1]: tcp/udp/fens ...
	 * argv[2]: tcp_ip前置地址/udp_ip组播地址
	 * argv[3]: Interface_ip 组播接收地址
	 * argv[4]:	fens::FensEnvID 环境标识
	 * argv[5]:	fens::FensNodeID 柜台节点
	 * 默认连仿真:		./xmddemo
	 * 指定TCP地址:		./xmddemo tcp tcp://210.14.72.21:4402
	 * 指定FENS地址:	./xmddemo fens tcp://210.14.72.21:42370 stock sim
	 * 指定组播地址:	./xmddemo udp udp://224.224.1.3:7880 x.x.x.x
	 * 实盘实时快照:	./xmddemo lob udp://224.224.1.3:7880 udp://224.224.3.3:7888 x.x.x.x
	 * 上述x.x.x.x使用托管服务器中接收Level1或期权行情的网口地址
	 * ******************************************************************************************/

	/*******************************************************************************
	 * 7*24小时只做技术调试用途，无法验证业务完整性，历史某日行情播放四次
	 * 仿真环境可用时间同交易所交易时间，撮合成交，行情之后一帧，即3秒
	 * 仿真环境和7*24小时环境仅支持TCP方式，在互联网环境可用，托管机房无法使用
	 * 仿真环境和7*24小时环境都支持Front直接连接和FENS名字服务器方式
	 * 非FENS方式时，API直接连行情前置。使用接口RegisterFront
	 * FENS方式时，注册名字服务器地址。使用接口RegisterNameServer
	 * 使用名字服务器的好处是当系统部署方式发生调整时外围终端无需做任何前置地址修改
	 * *****************************************************************************/

	CTORATstpXMdApi *api;
	char XMD_FENS_FrontAddress[64];
	char XMD_TCP_FrontAddress[64];
	char XMD_MCAST_FrontAddress[64];
	char XMD_MCAST_InterfaceIP[64];
	char XMD_MCAST_DeriveAddress[64];
	char XDM_FENS_FensEnvID[32];
	char XDM_FENS_FensNodeID[32];

	printf("XMdApiVersion:[%s]\n", CTORATstpXMdApi::GetApiVersion());
	if (argc == 1) //默认TCP连接仿真环境
	{
		strcpy(XMD_TCP_FrontAddress, "tcp://210.14.72.21:4402");
	}
	else if (argc == 3 && !strcmp(argv[1], "tcp")) //普通TCP方式
	{
		strcpy(XMD_TCP_FrontAddress, argv[2]);
	}
	else if (argc == 4 && !strcmp(argv[1], "udp")) // UDP 组播
	{
		strcpy(XMD_MCAST_FrontAddress, argv[2]); //组播地址
		strcpy(XMD_MCAST_InterfaceIP, argv[3]);	 //组播接收地址
	}
	else if (argc == 5 && !strcmp(argv[1], "fens")) // FENS名字服务器 TCP方式
	{
		strcpy(XMD_FENS_FrontAddress, argv[2]); // FENS 名字服务器地址
		strcpy(XDM_FENS_FensEnvID, argv[3]);	//注册FENS服务信息必需柜台环境类型，股票现货为"stock"
		strcpy(XDM_FENS_FensNodeID, argv[4]);	//仿真为“sim_xmd”，7*24小时“24_xmd”,支持FENS的生产环境一般为节点号
	}
	else if (argc == 5 && !strcmp(argv[1], "lob")) //同时组播订阅普通行情和衍生行情（合成快照），仅适用于生产环境
	{
		strcpy(XMD_MCAST_FrontAddress, argv[2]);  //普通服务组播地址
		strcpy(XMD_MCAST_DeriveAddress, argv[3]); //衍生服务组播地址
		strcpy(XMD_MCAST_InterfaceIP, argv[4]);	  //接收普通行情服务和衍生行情服务的网口地址(托管服务器)
	}

	/*************************创建实例 注册服务*****************/
	if (argc == 1 || !strcmp(argv[1], "tcp")) //默认或TCP方式
	{
		printf("************* XMD TCP *************\n");
		// TCP订阅lv1行情，前置Front和FENS方式都用默认构造
		api = CTORATstpXMdApi::CreateTstpXMdApi();
		api->RegisterFront(XMD_TCP_FrontAddress);
		printf("XMD_TCP_FrontAddress[TCP]::%s\n", XMD_TCP_FrontAddress);
	}
	else if (!strcmp(argv[1], "udp")) //组播普通行情
	{
		printf("************* XMD UDP *************\n");
		// XMD组播订阅lv1行情
		api = CTORATstpXMdApi::CreateTstpXMdApi(TORA_TSTP_MST_MCAST);
		// api->RegisterMulticast(XMD_MCAST_FrontAddress, XMD_MCAST_InterfaceIP, NULL);
		// udp://224.224.1.11:7880;10.188.82.150
		// let source_ip = "10.188.82.7";
		api->RegisterMulticast("udp://224.224.1.11:7880", "10.188.82.150", "10.188.82.7");
		printf("RegisterMulticast udp://224.224.1.11:7880;10.188.82.150\n");
		// printf("XMD_MCAST_FrontAddress[UDP]::%s\n", XMD_MCAST_FrontAddress);
	}
	else if (!strcmp(argv[1], "fens")) // FENS 名字服务注册
	{
		printf("********** XMD FENS MultiCast **********\n");
		/********************************************************************************
		 * 注册 fens 地址前还需注册 fens 用户信息，包括环境编号、节点编号、Fens 用户代码等信息
		 * 使用名字服务器的好处是当券商系统部署方式发生调整时外围终端无需做任何前置地址修改
		 * *****************************************************************************/
		// TCP订阅lv1行情，前置Front和FENS方式都用默认构造
		api = CTORATstpXMdApi::CreateTstpXMdApi();

		CTORATstpFensUserInfoField fens_user_info_field;
		memset(&fens_user_info_field, 0, sizeof(fens_user_info_field));
		// strcpy(fens_user_info_field.FensEnvID, XDM_FENS_FensEnvID);   //必填项，暂时固定为“stock”表示普通现货柜台
		// strcpy(fens_user_info_field.FensNodeID, XDM_FENS_FensNodeID); //必填项，生产环境需按实际填写,仿真环境为sim_xmd
		// api->RegisterFensUserInfo(&fens_user_info_field);
		//必须先注册Fens信息再注册Fens

		// api->RegisterNameServer(XMD_FENS_FrontAddress);
		api->RegisterNameServer("tcp://10.188.82.191:53270");
		printf("RegisterNameServer 10.188.82.191:53270\n");
		// printf("XMD_FENS_FrontAddress[FENS]::%s\n", XMD_FENS_FrontAddress);
	}
	else if (!strcmp(argv[1], "lob")) //组播普通+组播衍生行情(实时合成快照)
	{
		printf("************* XMD UDP+UDP *************\n");
		//组播订阅lv1行情及组播订阅合成快照
		api = CTORATstpXMdApi::CreateTstpXMdApi(TORA_TSTP_MST_MCAST, TORA_TSTP_MST_MCAST);
		//先注册普通服务，再注册衍生服务
		api->RegisterMulticast(XMD_MCAST_FrontAddress, XMD_MCAST_InterfaceIP, NULL);
		api->RegisterDeriveMulticast(XMD_MCAST_DeriveAddress, XMD_MCAST_InterfaceIP, NULL);
		printf("XMD_MCAST_FrontAddress[lob]::%s\n", XMD_MCAST_FrontAddress);
		printf("XMD_MCAST_DeriveAddress[lob]::%s\n", XMD_MCAST_DeriveAddress);
	}
	else
	{
		printf("/*********************************************demo运行说明************************************\n");
		printf("* argv[1]: tcp udp fens lob\t\t\t\t=[%s]\n", argv[1]);
		printf("* argv[2]: tcp/fens::FrontIP upd/lob::MCAST_IP\t\t=[%s]\n", argv[2]);
		printf("* argv[3]: fens::EnvID udp::InterfaceIP lob::DeriveIP\t=[%s]\n", argv[3]);
		printf("* argv[4]: fens::FensNodeID\t\t\t\t=[%s]\n", argv[4]);
		printf("* Usage:\n");
		printf("* 默认连仿真:		./xmddemo\n");
		printf("* 指定TCP地址:		./xmddemo tcp tcp://210.14.72.21:4402\n");
		printf("* 指定FENS地址:		./xmddemo fens tcp://210.14.72.21:42370 stock sim_xmd\n");
		printf("* 指定组播地址:		./xmddemo udp udp://224.224.1.3:7880 x.x.x.x\n");
		printf("* 实盘实时快照:		./xmddemo lob udp://224.224.1.3:7880 udp://224.224.3.3:7888 x.x.x.x\n");
		printf("* 上述x.x.x.x使用托管服务器中接收XMD行情的网口IP地址\n");
		printf("* ******************************************************************************************/\n");

		return 0;
	}

	DemoMdSpi spi(api);
	api->RegisterSpi(&spi);
	api->Init();
	getchar(); //控制台输入任意字符即退出
	// api->Join(); //阻塞线程，一般需要开启，否则当getchar遇到任意字符就退出了
	api->Release();
}
